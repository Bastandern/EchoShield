// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, StreamConfig};
use dasp_sample::conv::ToSample;
use hound::{SampleFormat as HoundSampleFormat, WavSpec, WavWriter};
use directories::BaseDirs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::BufWriter;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;
use tauri::State;
use tauri::Window;
use lazy_static::lazy_static;
use serde::Serialize;

mod db;

use db::{AudioFile, Database, User};

/// 前端波形绘制事件的数据载荷：ori=原始采样, new=保护后采样
#[derive(Clone, Serialize)]
struct Payload<T> {
    ori: Vec<T>,
    new: Vec<T>,
}

/// 扰动衰减系数 (0.0~1.0)，值越大保护越强但音质越差
const DISTURBANCE_ALPHA: f32 = 1.0;

lazy_static! {
    /// 音频处理线程是否运行中
    static ref HAS_RUN_AUDIO: AtomicBool = AtomicBool::new(false);
    /// 是否将保护后音频输出到虚拟声卡
    static ref SHOULD_PLAY_AUDIO_OUTPUT: AtomicBool = AtomicBool::new(false);
}

struct AppState {
    db: Mutex<Database>,
}

// ---- 用户认证 ----

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

// ---- 音频处理核心 ----

/// 启动实时音频保护：捕获麦克风 → 叠加对抗性扰动 → 输出到虚拟声卡
#[tauri::command]
async fn audio_process(
    state: State<'_, AppState>,
    user_id: String,
    add_values: Vec<f32>,
    window: Window,
) -> Result<(), String> {
    HAS_RUN_AUDIO.store(true, Ordering::SeqCst);
    SHOULD_PLAY_AUDIO_OUTPUT.store(false, Ordering::SeqCst);
    println!("Audio processing started - output is muted by default");

    // 毫秒时间戳作为录音文件名
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_millis() as u64;
    let filename = in_ms.to_string();

    state
        .db
        .lock()
        .unwrap()
        .save_audio_file(&user_id, &filename)
        .map_err(|e| e.to_string())?;

    // 音频设备操作在独立线程中执行，避免阻塞主线程
    thread::spawn(move || {
        let base_dirs = BaseDirs::new().unwrap();
        let path = base_dirs.data_dir().to_str().unwrap();
        let input_filepath = format!("{}/top.echoshield/waves/{}_ori.wav", path, filename);
        let output_filepath = format!("{}/top.echoshield/waves/{}_new.wav", path, filename);

        let host = cpal::default_host();

        let input_device = host
            .default_input_device()
            .expect("Failed to get default input device");

        // 优先使用虚拟音频线缆，找不到则回退默认输出
        let mut output_device = host
            .default_output_device()
            .expect("Failed to get default output device");

        let output_device_list = host.output_devices().unwrap();
        let mut virtual_device_found = false;
        for device in output_device_list {
            let device_name = device.name().unwrap();
            println!("Found output device: {}", device_name);
            if device_name == "Line 1 (Virtual Audio Cable)" {
                output_device = device;
                virtual_device_found = true;
                println!("Selected virtual audio device: {}", device_name);
                break;
            }
        }

        if !virtual_device_found {
            println!("WARNING: Virtual audio device 'Line 1 (Virtual Audio Cable)' not found!");
            println!("Available devices:");
            for device in host.output_devices().unwrap() {
                println!("  - {}", device.name().unwrap());
            }
            println!("Falling back to default output device");
        }

        println!("Using output device: {}", output_device.name().unwrap());
        println!("Start Audio Process");

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

        // 共享缓冲区：输入回调写入，输出回调读取
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let add_index = Arc::new(Mutex::new(0));

        let input_err_fn = |err| eprintln!("an error occurred on the input audio stream: {}", err);
        let input_sample_format = input_supported_configs.sample_format();
        let input_config: StreamConfig = input_supported_configs.into();
        let input_buffer = Arc::clone(&buffer);
        let input_add_index = Arc::clone(&add_index);

        println!("声道数：{}", input_config.channels);
        println!("采样率：{}", input_config.sample_rate.0);

        // WAV 写入器，3倍降采样以节省空间
        let input_spec = WavSpec {
            channels: input_config.channels,
            sample_rate: input_config.sample_rate.0 / 3,
            bits_per_sample: 16,
            sample_format: HoundSampleFormat::Int,
        };
        let mut input_writer = Arc::new(Mutex::new(
            WavWriter::create(input_filepath, input_spec).expect("Failed to create WAV file"),
        ));

        let output_err_fn =
            |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let output_sample_format = output_supported_configs.sample_format();
        let output_config: StreamConfig = output_supported_configs.into();
        let output_buffer = Arc::clone(&buffer);

        println!("Output device config:");
        println!("  Sample rate: {}", output_config.sample_rate.0);
        println!("  Channels: {}", output_config.channels);
        println!("  Sample format: {:?}", output_sample_format);

        let output_spec = WavSpec {
            channels: input_config.channels,
            sample_rate: output_config.sample_rate.0 / 3,
            bits_per_sample: 16,
            sample_format: HoundSampleFormat::Int,
        };
        let output_writer = Arc::new(Mutex::new(
            WavWriter::create(output_filepath, output_spec)
                .expect("Failed to create output WAV file"),
        ));

        let add_values_clone = add_values.clone();
        let mut output_writer_clone = Arc::clone(&output_writer);

        // 根据声卡采样格式构建输入流（Rust 要求编译期确定泛型，所以每种格式写一个分支）
        let input_stream = match input_sample_format {
            SampleFormat::F32 => input_device.build_input_stream(
                &input_config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    input_callback::<f32>(
                        data,
                        &input_buffer,
                        &input_add_index,
                        &add_values_clone,
                        &mut input_writer,
                        &window,
                        &mut output_writer_clone,
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
                        &add_values_clone,
                        &mut input_writer,
                        &window,
                        &mut output_writer_clone,
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
                        &add_values_clone,
                        &mut input_writer,
                        &window,
                        &mut output_writer_clone,
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
                        &add_values_clone,
                        &mut input_writer,
                        &window,
                        &mut output_writer_clone,
                    );
                },
                input_err_fn,
                None,
            ),
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        }
        .unwrap();

        let output_stream = match output_sample_format {
            SampleFormat::F32 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    output_callback::<f32>(data, &output_buffer);
                },
                output_err_fn,
                None,
            ),
            SampleFormat::I16 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                    output_callback::<i16>(data, &output_buffer);
                },
                output_err_fn,
                None,
            ),
            SampleFormat::U16 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                    output_callback::<u16>(data, &output_buffer);
                },
                output_err_fn,
                None,
            ),
            SampleFormat::U8 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [u8], _: &cpal::OutputCallbackInfo| {
                    output_callback::<u8>(data, &output_buffer);
                },
                output_err_fn,
                None,
            ),
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        }
        .unwrap();

        input_stream.play().unwrap();
        output_stream.play().unwrap();

        // 每秒检查一次运行标志，audio_stop 会将其设为 false
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

/// 输入回调：对每个采样点叠加对抗性扰动
///   modified = original + add_values[index] * DISTURBANCE_ALPHA
/// 同时推送波形数据给前端、写入WAV（3倍降采样）、放入共享缓冲区供输出回调使用
fn input_callback<T>(
    data: &[T],
    buffer: &Arc<Mutex<Vec<f32>>>,
    add_index: &Arc<Mutex<usize>>,
    add_values: &[f32],
    writer: &mut Arc<Mutex<WavWriter<BufWriter<File>>>>,
    window: &Window,
    output_writer: &mut Arc<Mutex<WavWriter<BufWriter<File>>>>,
) where
    T: Sample + ToSample<f32>,
{
    let mut buffer = buffer.lock().unwrap();
    let mut add_index = add_index.lock().unwrap();
    let mut add_index_channel = 0;

    let mut data_ori: Vec<f32> = Vec::new();
    let mut data_new: Vec<f32> = Vec::new();
    let mut save_index: i8 = 0;

    for &sample in data.iter() {
        save_index += 1;
        let normalized_sample = sample.to_sample::<f32>();

        // ★ 核心扰动叠加 ★
        let modified_sample = normalized_sample + (add_values[*add_index] * DISTURBANCE_ALPHA);

        data_ori.push(normalized_sample);
        data_new.push(modified_sample);
        buffer.push(modified_sample);

        // 立体声：左右声道共用同一个扰动值，每2个采样点前进一个扰动索引
        add_index_channel += 1;
        if add_index_channel == 2 {
            add_index_channel = 0;
            *add_index = (*add_index + 1) % add_values.len();
        }

        // 3倍降采样写入WAV
        if save_index == 3 {
            let sample_i16 = (normalized_sample * std::i16::MAX as f32)
                .clamp(std::i16::MIN as f32, std::i16::MAX as f32)
                as i16;
            writer
                .lock()
                .unwrap()
                .write_sample(sample_i16)
                .expect("Failed to write sample");

            let new_sample_i16 = (modified_sample * std::i16::MAX as f32)
                .clamp(std::i16::MIN as f32, std::i16::MAX as f32)
                as i16;
            if let Ok(mut writer_locked) = output_writer.lock() {
                writer_locked
                    .write_sample(new_sample_i16)
                    .expect("Failed to write sample to output.wav");
            }

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

/// 输出回调：从共享缓冲区取出保护后采样发送到虚拟声卡，未开启播放时输出静音
fn output_callback<T>(
    data: &mut [T],
    buffer: &Arc<Mutex<Vec<f32>>>,
) where
    T: Sample + FromSample<f32>,
{
    let mut buffer_lock = buffer.lock().unwrap();
    let should_play_actually = SHOULD_PLAY_AUDIO_OUTPUT.load(Ordering::SeqCst);

    for sample_device_output in data.iter_mut() {
        if should_play_actually {
            if let Some(processed_sample_f32) = buffer_lock.pop() {
                *sample_device_output = T::from_sample(processed_sample_f32);
            } else {
                *sample_device_output = Sample::EQUILIBRIUM;
            }
        } else {
            *sample_device_output = Sample::EQUILIBRIUM;
        }
    }
}

// ---- 播放控制 ----

#[tauri::command]
fn start_audio_playback() -> Result<(), String> {
    if !HAS_RUN_AUDIO.load(Ordering::SeqCst) {
        return Err("Audio processing is not running. Call audio_process first.".to_string());
    }
    println!("Command: start_audio_playback received. Enabling audio output to virtual audio device.");
    SHOULD_PLAY_AUDIO_OUTPUT.store(true, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
fn pause_audio_playback() -> Result<(), String> {
    println!("Command: pause_audio_playback received. Disabling audio output to virtual audio device (muting).");
    SHOULD_PLAY_AUDIO_OUTPUT.store(false, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
fn audio_stop() -> () {
    println!("Command: audio_stop received. Stopping audio processing thread and muting output.");
    HAS_RUN_AUDIO.store(false, Ordering::SeqCst);
    SHOULD_PLAY_AUDIO_OUTPUT.store(false, Ordering::SeqCst);
}

// ---- 录音文件管理 ----

#[tauri::command]
async fn get_user_audio_files(
    state: State<'_, AppState>,
    user_id: String,
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
    filename: String,
) -> Result<bool, String> {
    state
        .db
        .lock()
        .unwrap()
        .check_audio_file_access(&user_id, &filename)
        .map_err(|e| e.to_string())
}

/// 删除录音：先删数据库记录，再删磁盘文件（WAV + ASR转写文本）
#[tauri::command]
async fn delete_audio_file(
    state: State<'_, AppState>,
    user_id: String,
    filename: String,
) -> Result<bool, String> {
    let deleted = state
        .db
        .lock()
        .unwrap()
        .delete_audio_file(&user_id, &filename)
        .map_err(|e| e.to_string())?;

    if deleted {
        let base_dirs = BaseDirs::new().unwrap();
        let path = base_dirs.data_dir().to_str().unwrap();

        let ori_path = format!("{}/top.echoshield/waves/{}_ori.wav", path, filename);
        let _ = std::fs::remove_file(ori_path);

        let new_path = format!("{}/top.echoshield/waves/{}_new.wav", path, filename);
        let _ = std::fs::remove_file(new_path);

        let ori_text_path = format!("{}/top.echoshield/waves/{}_ori.txt", path, filename);
        let _ = std::fs::remove_file(ori_text_path);
        let new_text_path = format!("{}/top.echoshield/waves/{}_new.txt", path, filename);
        let _ = std::fs::remove_file(new_text_path);
    }

    Ok(deleted)
}

/// 重命名录音：先更新数据库，再重命名磁盘文件
#[tauri::command]
async fn rename_audio_file(
    state: State<'_, AppState>,
    user_id: String,
    new_filename: String,
    old_filename: String,
) -> Result<bool, String> {
    let renamed = state
        .db
        .lock()
        .unwrap()
        .rename_audio_file(&user_id, &old_filename, &new_filename)
        .map_err(|e| e.to_string())?;

    if renamed {
        let base_dirs = BaseDirs::new().unwrap();
        let path = base_dirs.data_dir().to_str().unwrap();

        let old_ori_path = format!("{}/top.echoshield/waves/{}_ori.wav", path, old_filename);
        let new_ori_path = format!("{}/top.echoshield/waves/{}_ori.wav", path, new_filename);
        let _ = std::fs::rename(old_ori_path, new_ori_path);

        let old_new_path = format!("{}/top.echoshield/waves/{}_new.wav", path, old_filename);
        let new_new_path = format!("{}/top.echoshield/waves/{}_new.wav", path, new_filename);
        let _ = std::fs::rename(old_new_path, new_new_path);

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

// ---- 应用入口 ----

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
