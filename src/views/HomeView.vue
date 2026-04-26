<script setup>
import router from '../router';
import { useI18n } from 'vue-i18n';
import Cookies from 'js-cookie';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { useUserStore } from '../stores/user';
import NavBar from '../components/NavBar.vue';

const { locale } = useI18n();

const isOpen = ref(false);

const changeLanguage = (lang) => {
  locale.value = lang;
  Cookies.set('language', lang);
  isOpen.value = false;
}

const btnStart = () => {
  router.push({ name: 'protect' })
}
const btnList = () => {
  router.push({ name: 'list' })
}
const btnAsync = () => {
  router.push({ name: 'async' })
}
const btnConfig = () => {
  router.push({ name: 'config' })
}

const userStore = useUserStore();
const isRecording = ref(false);
const addValues = ref([]);

const startRecord = async () => {
  if (!isRecording.value) {
    isRecording.value = true;
    await invoke('audio_process', {
      userId: userStore.user.id,
      addValues: addValues.value
    });
  }
};
</script>

<template>
  <div class="container">
    <div class="background-ellipse"></div>
    <div class="background-ellipse-second"></div>
    <div class="background-ellipse-right"></div>
    <div class="background-ellipse-second-right"></div>

    <NavBar />

    <div class="main-content">
      <div class="logo-container fade-in-up">
        <img src="/images/logo-text2.png" alt="AudioShield" class="main-logo">
      </div>

      <div class="buttons-container fade-in-up">
        <button class="action-button primary fade-in-up" @click="btnStart" :class="{ 'en-size': locale === 'en' }">
          <img src="/src-tauri/icons/icon1.png" alt="start" class="button-icon">
          <span class="button-text">{{ $t('home.btnStart') }}</span>
        </button>
        <button class="action-button secondary fade-in-up" @click="btnList" :class="{ 'en-size': locale === 'en' }">
          <img src="/src-tauri/icons/icon2.png" alt="history" class="button-icon">
          <span class="button-text">{{ $t('home.btnList') }}</span>
        </button>
        <button class="action-button secondary fade-in-up" @click="btnAsync" :class="{ 'en-size': locale === 'en' }">
          <img src="/src-tauri/icons/icon3.png" alt="sync" class="button-icon">
          <span class="button-text">{{ $t('home.btnAsync') }}</span>
        </button>
        <button class="action-button secondary fade-in-up" @click="btnConfig" :class="{ 'en-size': locale === 'en' }">
          <img src="/src-tauri/icons/icon4.png" alt="config" class="button-icon">
          <span class="button-text">{{ $t('home.btnConfig') }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  position: relative;
  width: 100vw;
  height: 100vh;
  min-width: 800px;
  min-height: 600px;
  background: #FFFFFF;
  display: flex;
  flex-direction: column;
  padding: 0;
  overflow: hidden;
  z-index: 0;
}

.background-ellipse {
  position: absolute;
  width: 600px;
  height: 400px;
  left: -400px;
  bottom: -150px;
  background: #FD9F86;
  border-radius: 50%;
  filter: blur(100px);
  opacity: 1;
  pointer-events: none;
  z-index: 2;
}

.background-ellipse-second {
  position: absolute;
  width: 600px;
  height: 400px;
  left: -5vw;
  bottom: -250px;
  background: #FCCE88;
  border-radius: 50%;
  filter: blur(100px);
  opacity: 1;
  pointer-events: none;
  z-index: 1;
}

.background-ellipse-right {
  position: absolute;
  width: 600px;
  height: 400px;
  right: -400px;
  bottom: -150px;
  background: #00FDEE;
  border-radius: 50%;
  filter: blur(100px);
  opacity: 1;
  pointer-events: none;
  z-index: 2;
}

.background-ellipse-second-right {
  position: absolute;
  width: 600px;
  height: 400px;
  right: -5vw;
  bottom: -250px;
  background: #B76CC8;
  border-radius: 50%;
  filter: blur(100px);
  opacity: 0.8;
  pointer-events: none;
  z-index: 1;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding-top: min(4vh, 30px);
  min-height: 400px;
  width: 100%;
  padding-bottom: min(80px, 10vh);
  margin-top: 80px;
}

.logo-container {
  width: 100%;
  display: flex;
  justify-content: center;
  margin-bottom: min(40px, 5vh);
  padding: 0 20px;
  margin-top: 0;
}

.main-logo {
  width: 72%;
  height: auto;
  object-fit: contain;
  max-width: 1100px;
  min-width: 480px;
  display: block;
  margin: 0 auto;
  transition: transform 0.5s ease;
}

.main-logo:hover {
  transform: scale(1.05);
}

.buttons-container {
  display: flex;
  flex-direction: column;
  gap: min(28px, 3.5vh);
  width: 100%;
  align-items: center;
  padding: 0;
  margin-top: 20px;
  position: relative;
  z-index: 2;
}

.action-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 38%;
  min-width: 360px;
  padding: clamp(16px, 2.5vh, 24px) clamp(20px, 3vw, 30px);
  border: none;
  border-radius: 15px;
  font-size: clamp(16px, 2.5vw, 22px);
  font-weight: 600;
  cursor: pointer;
  transition: all 0.5s ease;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  min-height: 56px;
  gap: 15px;

  &.en-size {
    min-width: 520px;
    width: 52%;
    padding-left: clamp(25px, 3.5vw, 35px);
    padding-right: clamp(25px, 3.5vw, 35px);
    gap: 20px;
  }

  &.primary {
    background-color: #4294EC;
    color: #FFFFFF;
  }

  &.secondary {
    background-color: #B0DAF3;
    color: #214D89;
  }

  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2);
  }
}

.button-icon {
  width: 28px;
  height: 28px;
  object-fit: contain;
  flex-shrink: 0;
}

.en-size .button-icon {
  width: 32px;
  height: 32px;
}

.button-text {
  text-align: center;
  flex-grow: 1;
  letter-spacing: 0.5px;
  white-space: nowrap;
}

.lang-select {
  position: relative;
  cursor: pointer;
  z-index: 100;
  padding-top: 4px;
  padding-right: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.lang-text {
  color: #000000;
  font-weight: 700;
  font-size: 18px;
  text-align: center;
}

.options {
  position: absolute;
  top: 100%;
  right: 20px;
  margin-top: 8px;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  min-width: 90px;
  transform: translateX(50%);
}

.option {
  padding: 8px 16px;
  cursor: pointer;
  white-space: nowrap;
  font-size: 16px;
  color: #000000;
  font-weight: 600;
  text-align: center;
  transition: background-color 0.3s ease;

  &:hover {
    background: rgba(0, 0, 0, 0.1);
    transform: scale(1.02);
  }
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
    transform: scale(0.9);
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

.logo-container.fade-in-up {
  animation: scaleIn 1s ease-out 0.2s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.logo-container.fade-in-up .main-logo:hover {
  transform: scale(1.05) !important;
}

.buttons-container.fade-in-up {
  animation: fadeIn-Up 0.8s ease-out 0.4s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.action-button.fade-in-up:nth-child(1) {
  animation: slideIn-Left 0.6s ease-out 0.6s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.action-button.fade-in-up:nth-child(2) {
  animation: slideIn-Right 0.6s ease-out 0.8s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.action-button.fade-in-up:nth-child(3) {
  animation: slideIn-Left 0.6s ease-out 1.0s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.action-button.fade-in-up:nth-child(4) {
  animation: slideIn-Right 0.6s ease-out 1.2s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.action-button.fade-in-up:hover {
  transform: translateY(-4px) !important;
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2) !important;
}

.option:hover {
  background: rgba(0, 0, 0, 0.1) !important;
  transform: scale(1.02) !important;
}

.background-ellipse {
  animation: fadeIn-Up 1s ease-out 0.1s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.background-ellipse-second {
  animation: fadeIn-Up 1s ease-out 0.3s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.background-ellipse-right {
  animation: fadeIn-Up 1s ease-out 0.2s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.background-ellipse-second-right {
  animation: fadeIn-Up 1s ease-out 0.4s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}
</style>
