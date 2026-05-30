<script setup>
import { invoke } from '@tauri-apps/api/tauri';
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { exists, createDir, BaseDirectory, readTextFile } from '@tauri-apps/api/fs';
import { listen } from '@tauri-apps/api/event';
import NavBar from '../components/NavBar.vue';
import ReturnButton from '../components/ReturnButton.vue';
import { useUserStore } from '../stores/user';

const userStore = useUserStore();
const uapDetected = ref(false);
const isPlaying = ref(false); // 默认暂停状态

// Start protection
const startProtect = async () => {
  if (!await exists('uap', { dir: BaseDirectory.AppData })) {
    // Create folder if it doesn't exist
    uapDetected.value = false;
    console.log('create dir');
    await createDir('uap', { dir: BaseDirectory.AppData, recursive: true });
  } else if (!await exists('uap/uap.ar', { dir: BaseDirectory.AppData })) {
    // UAP not found
    uapDetected.value = false;
  } else {
    // UAP found, load it
    uapDetected.value = true;
    const uapFileData = await readTextFile('uap/uap.ar', { dir: BaseDirectory.AppData });
    const lines = uapFileData.trim().split('\n');

    // Convert each line to a float and add to an array
    const floatArray = lines.map(line => {
      const num = parseFloat(line);
      if (!isNaN(num)) {
        return num;
      } else {
        console.warn(`"${line}" cannot be converted to a float.`);
        return null;
      }
    }).filter(num => num !== null); // Filter out failed conversions

    // Start audio protection (默认暂停状态)
    invoke('audio_process', { 
      userId: userStore.user.id,
      addValues: floatArray.slice(0, 17000) 
    });
  }
}

let listenFromBackend;
const canvasOriRef = ref();
const canvasNewRef = ref();

// Draw waveform
const drawWave = async (data, canvasRef) => {
  const context = canvasRef.value.getContext('2d');
  const width = canvasRef.value.width;
  const height = canvasRef.value.height;
  const step = data.length / width;
  context.strokeStyle = '#214D89';
  context.lineWidth = 2;
  context.clearRect(0, 0, width, height);
  context.beginPath();
  context.moveTo(0, height / 2);
  for (let i = 0; i < width; i++) {
    const sample = data[Math.ceil(i * step)];
    const y = sample * (height / 2) + (height / 2);
    context.lineTo(i, y);
  }
  context.stroke();
}

// Initialize listener to get results from backend
const initListen = async () => {
  listenFromBackend = await listen('audio_update', (event) => {
    drawWave(event.payload.ori, canvasOriRef);
    drawWave(event.payload.new, canvasNewRef);
  });
}

onMounted(() => {
  startProtect();
  initListen();
});

onBeforeUnmount(() => {
  if (listenFromBackend) {
  listenFromBackend();
  }
});

// 返回前的清理函数
const beforeReturn = async () => {
  if (listenFromBackend) {
    await listenFromBackend();
  }
  // 添加停止音频处理
  await stopAudio();
};

// 切换音频播放状态
const toggleAudioPlayback = async () => {
  if (isPlaying.value) {
    await invoke('pause_audio_playback');
    isPlaying.value = false;
  } else {
    await invoke('start_audio_playback');
    isPlaying.value = true;
  }
};

// 停止音频处理
const stopAudio = async () => {
  await invoke('audio_stop');
  isPlaying.value = false;
};

</script>

<template>
  <div class="container">
    <!-- 导航栏 -->
    <NavBar />
    
    <div class="wave-box fade-in-up">
      <div class="wave-container fade-in-up">
        <div class="wave-title">{{ $t('protect.ori') }}</div>
        <div class="wave-display">
          <canvas ref="canvasOriRef"></canvas>
        </div>
      </div>
      <div class="wave-container fade-in-up">
        <div class="wave-title">{{ $t('protect.new') }}</div>
        <div class="wave-display">
          <canvas ref="canvasNewRef"></canvas>
        </div>
      </div>
    </div>
    <div class="button-group fade-in-up">
      <div class="play-button" @click="toggleAudioPlayback">
        <img :src="isPlaying ? '/icons/icon8.png' : '/icons/icon7.png'" class="play-icon" :alt="isPlaying ? 'Mute' : 'Play'" />
        <span class="play-text">{{ isPlaying ? $t('protect.mute') : $t('protect.play') }}</span>
      </div>
      <div class="stop-button" @click="stopAudio">
        <img src="/icons/icon6.png" class="stop-icon" alt="Stop" />
        <span class="stop-text">{{ $t('protect.stop') }}</span>
      </div>
      <ReturnButton :before-return="beforeReturn" />
    </div>
  </div>
</template>

<style scoped>
.container {
  padding: 0;
  background-color: #FFFFFF;
  box-sizing: border-box;
  width: 100vw;
  height: 100vh;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.wave-box {
  width: 100%;
  max-width: 1000px;
  display: flex;
  flex-direction: column;
  gap: 20px;
  margin-top: 80px;
  position: relative;
  z-index: 1;
  padding: 0 20px;
}

.wave-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.wave-title {
  color: #214D89;
  font-size: 1.5rem;
  font-weight: bold;
  margin-bottom: 15px;
  text-align: center;
}

.wave-display {
  position: relative;
  margin: 10px;
  width: 100%;
  padding: 20px;
  border-radius: 30px;
  background: #C9E4F4;
}

canvas {
  width: 100%;
  height: 25vh;
  border-radius: 12px;
  background: #C9E4F4;
}

.button-group {
  position: fixed;
  left: 20px;
  bottom: 20px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  z-index: 1000;
  pointer-events: auto;
}

.play-button {
  padding: 8px 20px;
  border-radius: 12px;
  background-color: #C9E4F4;
  color: #214D89;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 12px;
  user-select: none;
  min-width: 106px;
  justify-content: flex-start;
}

.play-icon {
  width: 20px;
  height: 20px;
  object-fit: contain;
  margin-left: -4px; 
}

.play-text {
  font-size: 16px;
  color: #214D89;
  flex: 1;
  text-align: left;
}

.play-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(201, 228, 244, 0.4);
}

.stop-button {
  padding: 8px 20px;
  border-radius: 12px;
  background-color: #C9E4F4;
  color: #214D89;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 12px;
  user-select: none;
  min-width: 106px;
  justify-content: flex-start;
}

.stop-icon {
  width: 20px;
  height: 20px;
  object-fit: contain;
  margin-left: -4px; 
}

.stop-text {
  font-size: 16px;
  color: #214D89;
  flex: 1;
  text-align: left;
}

.stop-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(201, 228, 244, 0.4);
}

:deep(.return) {
  position: static !important;
  margin: 0;
}

@keyframes fadeIn-Up {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes slideIn-Left {
  from {
    opacity: 0;
    transform: translateX(-30px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes slideIn-Right {
  from {
    opacity: 0;
    transform: translateX(30px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes scaleIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.fade-in-up {
  animation: fadeIn-Up 0.8s ease-out forwards;
  animation-fill-mode: forwards;
}

.wave-box.fade-in-up {
  animation: scaleIn 1s ease-out 0.2s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.wave-container.fade-in-up:nth-child(1) {
  animation: slideIn-Left 0.8s ease-out 0.4s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.wave-container.fade-in-up:nth-child(2) {
  animation: slideIn-Left 0.8s ease-out 0.6s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.button-group.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.8s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.wave-title {
  animation: fadeIn-Up 0.6s ease-out 0.5s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.wave-display {
  animation: scaleIn 0.8s ease-out 0.7s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

canvas {
  animation: fadeIn-Up 0.6s ease-out 0.9s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.play-button {
  animation: fadeIn-Up 0.6s ease-out 0.9s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.stop-button {
  animation: fadeIn-Up 0.6s ease-out 1.0s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}
</style>
