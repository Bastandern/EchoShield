// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, StreamConfig};
use dasp_sample::conv::ToSample;
use directories::BaseDirs;
use hound::{SampleFormat as HoundSampleFormat, WavSpec, WavWriter};
use lazy_static::lazy_static;
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Window;
use tauri::Manager;

mod db;

use db::{Database, User, AudioFile};
use tauri::State;

#[derive(Clone, Serialize)]
struct Payload<T> {
    ori: Vec<T>,
    new: Vec<T>,
}

// 扰动衰减系数，用于改善音频质量 (0.0-1.0)
// 值越小，扰动越小，音频质量越好，但保护效果可能减弱
// 值越大，扰动越大，保护效果越好，但音频质量可能下降
const DISTURBANCE_ALPHA: f32 = 0.7;

lazy_static! {
    static ref HAS_RUN_AUDIO: AtomicBool = AtomicBool::new(false);
    static ref SHOULD_PLAY_AUDIO_OUTPUT: AtomicBool = AtomicBool::new(false);
}

struct AppState {
    db: Mutex<Database>,
}

#[tauri::command]
async fn register(
    state: State<'_, AppState>,
    username: String,
    password: String,
) -> Result<User, String> {
    state
        .db
        .lock()
        .unwrap()
        .register_user(&username, &password)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn login(
    state: State<'_, AppState>,
    username: String,
    password: String,
) -> Result<Option<User>, String> {
    state
        .db
        .lock()
        .unwrap()
        .login_user(&username, &password)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn audio_process(
    state: State<'_, AppState>,
    user_id: String,
    add_values: Vec<f32>, 
    window: Window
) -> Result<(), String> {
    HAS_RUN_AUDIO.store(true, Ordering::SeqCst);
    // 默认暂停状态，用户需要手动点击播放
    SHOULD_PLAY_AUDIO_OUTPUT.store(false, Ordering::SeqCst);
    
    // 获取当前时间作为文件名
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let in_ms = since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_millis() as u64;
    let filename = in_ms.to_string();

    // 保存音频文件记录到数据库
    state
        .db
        .lock()
        .unwrap()
        .save_audio_file(&user_id, &filename)
        .map_err(|e| e.to_string())?;

    thread::spawn(move || {
        let base_dirs = BaseDirs::new().unwrap();
        let path = base_dirs.data_dir().to_str().unwrap();
        // 创建文件
        let input_filepath = format!("{}/top.echoshield/waves/{}_ori.wav", path, filename);
        let output_filepath = format!("{}/top.echoshield/waves/{}_new.wav", path, filename);

        // 获取默认的 host
        let host = cpal::default_host();

        // 获得默认输入输出设备
        let input_device = host
            .default_input_device()
            .expect("Failed to get default input device");

        let mut output_device = host
            .default_output_device()
            .expect("Failed to get default output device");

        let output_device_list = host.output_devices().unwrap();

        for device in output_device_list {
            if device.name().unwrap() == "Line 1 (Virtual Audio Cable)" {
                output_device = device;
            }
        }

        println!("Start Audio Process");

        // 获取输入输出配置
        let mut input_supported_configs_range = input_device
            .supported_input_configs()
            .expect("error while querying configs");

        let input_supported_configs = input_supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();

        let mut output_supported_configs_range = output_device
            .supported_output_configs()
            .expect("error while querying configs");

        let output_supported_configs = output_supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();

        // 音频数据缓冲区
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let add_index = Arc::new(Mutex::new(0));

        // 输入部分
        let input_err_fn = |err| eprintln!("an error occurred on the input audio stream: {}", err);
        let input_sample_format = input_supported_configs.sample_format();
        let input_config: StreamConfig = input_supported_configs.into();
        let input_buffer = Arc::clone(&buffer);
        let input_add_index = Arc::clone(&add_index);

        println!("声道数：{}", input_config.channels);
        println!("采样率：{}", input_config.sample_rate.0);

        // 创建 WAV 文件写入器
        let input_spec = WavSpec {
            channels: input_config.channels,
            sample_rate: input_config.sample_rate.0 / 3,
            bits_per_sample: 16,
            sample_format: HoundSampleFormat::Int,
        };
        let mut input_writer = Arc::new(Mutex::new(
            WavWriter::create(input_filepath, input_spec).expect("Failed to create WAV file"),
        ));

        // 创建输入流
        let input_stream = match input_sample_format {
            SampleFormat::F32 => input_device.build_input_stream(
                &input_config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    input_callback::<f32>(
                        data,
                        &input_buffer,
                        &input_add_index,
                        &add_values,
                        &mut input_writer,
                        &window,
                    );
                },
                input_err_fn,
                None,
            ),
            SampleFormat::I16 => input_device.build_input_stream(
                &input_config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    input_callback::<i16>(
                        data,
                        &input_buffer,
                        &input_add_index,
                        &add_values,
                        &mut input_writer,
                        &window,
                    );
                },
                input_err_fn,
                None,
            ),
            SampleFormat::U16 => input_device.build_input_stream(
                &input_config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    input_callback::<u16>(
                        data,
                        &input_buffer,
                        &input_add_index,
                        &add_values,
                        &mut input_writer,
                        &window,
                    );
                },
                input_err_fn,
                None,
            ),
            SampleFormat::U8 => input_device.build_input_stream(
                &input_config,
                move |data: &[u8], _: &cpal::InputCallbackInfo| {
                    input_callback::<u8>(
                        data,
                        &input_buffer,
                        &input_add_index,
                        &add_values,
                        &mut input_writer,
                        &window,
                    );
                },
                input_err_fn,
                None,
            ),
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        }
        .unwrap();

        // 输出部分
        let output_err_fn =
            |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let output_sample_format = output_supported_configs.sample_format();
        let output_config: StreamConfig = output_supported_configs.into();
        let output_buffer = Arc::clone(&buffer);

        // 创建 WAV 文件写入器
        let output_spec = WavSpec {
            channels: input_config.channels,
            sample_rate: output_config.sample_rate.0 / 3,
            bits_per_sample: 16,
            sample_format: HoundSampleFormat::Int,
        };
        let mut output_writer = Arc::new(Mutex::new(
            WavWriter::create(output_filepath, output_spec).expect("Failed to create WAV file"),
        ));

        // 创建输出流
        let output_stream = match output_sample_format {
            SampleFormat::F32 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    output_callback::<f32>(data, &output_buffer, &mut output_writer);
                },
                output_err_fn,
                None,
            ),
            SampleFormat::I16 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                    output_callback::<i16>(data, &buffer, &mut output_writer);
                },
                output_err_fn,
                None,
            ),
            SampleFormat::U16 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                    output_callback::<u16>(data, &buffer, &mut output_writer);
                },
                output_err_fn,
                None,
            ),
            SampleFormat::U8 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [u8], _: &cpal::OutputCallbackInfo| {
                    output_callback::<u8>(data, &buffer, &mut output_writer);
                },
                output_err_fn,
                None,
            ),
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        }
        .unwrap();

        input_stream.play().unwrap();
        output_stream.play().unwrap();

        loop {
            if HAS_RUN_AUDIO.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_secs(1));
            } else {
                println!("Quit thread");
                break;
            }
        }

        input_stream.pause().unwrap();
        output_stream.pause().unwrap();
    });

    Ok(())
}

fn input_callback<T>(
    data: &[T],
    buffer: &Arc<Mutex<Vec<f32>>>,
    add_index: &Arc<Mutex<usize>>,
    add_values: &[f32],
    writer: &mut Arc<Mutex<WavWriter<BufWriter<File>>>>,
    window: &Window,
) where
    T: Sample + ToSample<f32>,
{
    // 先上互斥锁，防止数据竞争
    let mut buffer = buffer.lock().unwrap();
    let mut add_index = add_index.lock().unwrap();
    let mut add_index_channel = 0;

    let mut data_ori: Vec<f32> = Vec::new();
    let mut data_new: Vec<f32> = Vec::new();
    let mut save_index: i8 = 0;

    // 对获取到的每一位音频数据进行逐位处理
    for &sample in data.iter() {
        save_index += 1;
        let normalized_sample = sample.to_sample::<f32>();
        
        // 使用扰动衰减系数来改善音频质量
        let modified_sample = normalized_sample + (add_values[*add_index] * DISTURBANCE_ALPHA);
        
        // 保存原始音频数据和处理过后的音频数据
        data_ori.push(normalized_sample);
        data_new.push(modified_sample);
        buffer.push(modified_sample);

        add_index_channel += 1;
        if add_index_channel == 2 {
            add_index_channel = 0;
            *add_index = (*add_index + 1) % add_values.len();
        }

        if save_index == 3 {
            // 将样本转换为16位整数并写入WAV文件
            let sample_i16 = (normalized_sample * std::i16::MAX as f32)
                .clamp(std::i16::MIN as f32, std::i16::MAX as f32)
                as i16;
            writer
                .lock()
                .unwrap()
                .write_sample(sample_i16)
                .expect("Failed to write sample");
            save_index = 0;
        }
    }

    let _ = window
        .emit(
            "audio_update",
            Payload {
                ori: data_ori,
                new: data_new,
            },
        )
        .unwrap();
}

fn output_callback<T>(
    data: &mut [T],
    buffer: &Arc<Mutex<Vec<f32>>>,
    writer: &mut Arc<Mutex<WavWriter<BufWriter<File>>>>,
) where
    T: Sample + FromSample<f32>,
{
    let mut buffer_lock = buffer.lock().unwrap();
    let mut save_index: i8 = 0;
    // 获取当前是否应该播放声音的状态
    let should_play_actually = SHOULD_PLAY_AUDIO_OUTPUT.load(Ordering::SeqCst);

    for sample_device_output in data.iter_mut() {
        if let Some(processed_sample_f32) = buffer_lock.pop() {
            // 始终将处理后的样本写入 _new.wav 文件
            save_index += 1;
            if save_index == 3 {
                let sample_i16 = (processed_sample_f32 * std::i16::MAX as f32)
                    .clamp(std::i16::MIN as f32, std::i16::MAX as f32)
                    as i16;
                if let Ok(mut writer_locked) = writer.lock() {
                    writer_locked
                        .write_sample(sample_i16)
                        .expect("Failed to write sample to new.wav");
                }
                save_index = 0;
            }

            // 根据 should_play_actually 标志决定是播放实际处理的声音还是静音
            if should_play_actually {
                *sample_device_output = T::from_sample(processed_sample_f32);
            } else {
                *sample_device_output = Sample::EQUILIBRIUM; // 输出静音
            }
        } else {
            // 如果缓冲区为空，也输出静音
            *sample_device_output = Sample::EQUILIBRIUM;
        }
    }
}

#[tauri::command]
fn start_audio_playback() -> Result<(), String> {
    if !HAS_RUN_AUDIO.load(Ordering::SeqCst) {
        return Err("Audio processing is not running. Call audio_process first.".to_string());
    }
    println!("Command: start_audio_playback received. Enabling audio output.");
    SHOULD_PLAY_AUDIO_OUTPUT.store(true, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
fn pause_audio_playback() -> Result<(), String> {
    if !HAS_RUN_AUDIO.load(Ordering::SeqCst) {
        // 即使音频处理未运行，设置此标志也无害，但通常在运行时控制
        // return Err("Audio processing is not running.".to_string());
    }
    println!("Command: pause_audio_playback received. Disabling audio output (muting).");
    SHOULD_PLAY_AUDIO_OUTPUT.store(false, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
fn audio_stop() -> () {
    println!("Command: audio_stop received. Stopping audio processing thread and muting output.");
    HAS_RUN_AUDIO.store(false, Ordering::SeqCst);
    SHOULD_PLAY_AUDIO_OUTPUT.store(false, Ordering::SeqCst);
}

#[tauri::command]
async fn get_user_audio_files(
    state: State<'_, AppState>,
    user_id: String
) -> Result<Vec<AudioFile>, String> {
    state
        .db
        .lock()
        .unwrap()
        .get_user_audio_files(&user_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_audio_file_access(
    state: State<'_, AppState>,
    user_id: String,
    filename: String
) -> Result<bool, String> {
    state
        .db
        .lock()
        .unwrap()
        .check_audio_file_access(&user_id, &filename)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_audio_file(
    state: State<'_, AppState>,
    user_id: String,
    filename: String,
) -> Result<bool, String> {
    // 删除数据库记录
    let deleted = state
        .db
        .lock()
        .unwrap()
        .delete_audio_file(&user_id, &filename)
        .map_err(|e| e.to_string())?;

    if deleted {
        // 删除实际的音频文件
        let base_dirs = BaseDirs::new().unwrap();
        let path = base_dirs.data_dir().to_str().unwrap();
        
        // 删除原始音频文件
        let ori_path = format!("{}/top.echoshield/waves/{}_ori.wav", path, filename);
        let _ = std::fs::remove_file(ori_path);
        
        // 删除处理后的音频文件
        let new_path = format!("{}/top.echoshield/waves/{}_new.wav", path, filename);
        let _ = std::fs::remove_file(new_path);
        
        // 删除相关的文本文件
        let ori_text_path = format!("{}/top.echoshield/waves/{}_ori.txt", path, filename);
        let _ = std::fs::remove_file(ori_text_path);
        let new_text_path = format!("{}/top.echoshield/waves/{}_new.txt", path, filename);
        let _ = std::fs::remove_file(new_text_path);
    }

    Ok(deleted)
}

#[tauri::command]
async fn rename_audio_file(
    state: State<'_, AppState>,
    user_id: String,
    old_filename: String,
    new_filename: String,
) -> Result<bool, String> {
    // 检查文件是否存在且属于该用户
    let renamed = state
        .db
        .lock()
        .unwrap()
        .rename_audio_file(&user_id, &old_filename, &new_filename)
        .map_err(|e| e.to_string())?;

    if renamed {
        // 重命名实际的音频文件
        let base_dirs = BaseDirs::new().unwrap();
        let path = base_dirs.data_dir().to_str().unwrap();
        
        // 重命名原始音频文件
        let old_ori_path = format!("{}/top.echoshield/waves/{}_ori.wav", path, old_filename);
        let new_ori_path = format!("{}/top.echoshield/waves/{}_ori.wav", path, new_filename);
        let _ = std::fs::rename(old_ori_path, new_ori_path);
        
        // 重命名处理后的音频文件
        let old_new_path = format!("{}/top.echoshield/waves/{}_new.wav", path, old_filename);
        let new_new_path = format!("{}/top.echoshield/waves/{}_new.wav", path, new_filename);
        let _ = std::fs::rename(old_new_path, new_new_path);
        
        // 重命名相关的文本文件
        let old_ori_text_path = format!("{}/top.echoshield/waves/{}_ori.txt", path, old_filename);
        let new_ori_text_path = format!("{}/top.echoshield/waves/{}_ori.txt", path, new_filename);
        let _ = std::fs::rename(old_ori_text_path, new_ori_text_path);
        
        let old_new_text_path = format!("{}/top.echoshield/waves/{}_new.txt", path, old_filename);
        let new_new_text_path = format!("{}/top.echoshield/waves/{}_new.txt", path, new_filename);
        let _ = std::fs::rename(old_new_text_path, new_new_text_path);
    }

    Ok(renamed)
}

#[tauri::command]
async fn delete_file(filepath: String) -> Result<(), String> {
    let base_dirs = BaseDirs::new().unwrap();
    let path = base_dirs.data_dir().to_str().unwrap();
    let full_path = format!("{}/top.echoshield/{}", path, filepath);
    
    std::fs::remove_file(&full_path).map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let config = app.config();
            let db = Database::new(&config).expect("Failed to initialize database");
            app.manage(AppState {
                db: Mutex::new(db),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register, 
            login, 
            audio_process, 
            audio_stop,
            start_audio_playback,
            pause_audio_playback,
            get_user_audio_files,
            check_audio_file_access,
            delete_audio_file,
            rename_audio_file,
            delete_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
