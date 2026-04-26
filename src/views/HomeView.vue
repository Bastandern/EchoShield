<!-- HomeView.vue - 主页面组件 -->
<script setup>
// 导入必要的依赖
import router from '../router';
import { useI18n } from 'vue-i18n';
import Cookies from 'js-cookie';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { useUserStore } from '../stores/user';
import NavBar from '../components/NavBar.vue';

// 获取国际化实例
const { locale } = useI18n();

// 控制语言选择下拉菜单的显示状态
const isOpen = ref(false);

// 语言切换函数
const changeLanguage = (lang) => {
  locale.value = lang;
  Cookies.set('language', lang);
  isOpen.value = false;  // 选择后关闭下拉菜单
}

// 按钮点击事件处理函数
const btnStart = () => {
  router.push({ name: 'protect' }) // 跳转到保护页面
}
const btnList = () => {
  router.push({ name: 'list' }) // 跳转到列表页面
}
const btnAsync = () => {
  router.push({ name: 'async' }) // 跳转到异步页面
}
const btnConfig = () => {
  router.push({ name: 'config' }) // 临时跳转到 404 页面
}

const userStore = useUserStore();
const isRecording = ref(false);
const addValues = ref([]);

const startRecord = async () => {
  if (!isRecording.value) {
    isRecording.value = true;
    // 调用后端开始录制
    await invoke('audio_process', { 
      userId: userStore.user.id,
      addValues: addValues.value 
    });
  }
};
</script>

<template>
  <!-- 主容器 -->
  <div class="container">
    <!-- 背景装饰 -->
    <div class="background-ellipse"></div>
    <div class="background-ellipse-second"></div>
    <div class="background-ellipse-right"></div>
    <div class="background-ellipse-second-right"></div>
    
    <!-- 导航栏 -->
    <NavBar />
    
    <!-- 主要内容区域 -->
    <div class="main-content">
      <!-- 大Logo展示区域 -->
      <div class="logo-container fade-in-up">
        <img src="/images/logo-text2.png" alt="AudioShield" class="main-logo">
      </div>
      
      <!-- 按钮组 -->
      <div class="buttons-container fade-in-up">
        <!-- 启动按钮 - 主要按钮 -->
        <button class="action-button primary fade-in-up" @click="btnStart" :class="{ 'en-size': locale === 'en' }">
          <img src="/src-tauri/icons/icon1.png" alt="start" class="button-icon">
          <span class="button-text">{{ $t('home.btnStart') }}</span>
        </button>
        <!-- 历史记录按钮 -->
        <button class="action-button secondary fade-in-up" @click="btnList" :class="{ 'en-size': locale === 'en' }">
          <img src="/src-tauri/icons/icon2.png" alt="history" class="button-icon">
          <span class="button-text">{{ $t('home.btnList') }}</span>
        </button>
        <!-- 同步按钮 -->
        <button class="action-button secondary fade-in-up" @click="btnAsync" :class="{ 'en-size': locale === 'en' }">
          <img src="/src-tauri/icons/icon3.png" alt="sync" class="button-icon">
          <span class="button-text">{{ $t('home.btnAsync') }}</span>
        </button>
        <!-- 配置按钮 -->
        <button class="action-button secondary fade-in-up" @click="btnConfig" :class="{ 'en-size': locale === 'en' }">
          <img src="/src-tauri/icons/icon4.png" alt="config" class="button-icon">
          <span class="button-text">{{ $t('home.btnConfig') }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 主容器样式 */
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

/* 背景装饰椭圆 */
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

/* 第二个背景装饰椭圆 */
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

/* 右侧背景装饰椭圆 */
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

/* 右侧第二个背景装饰椭圆 */
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

/* 主要内容区域样式 */
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding-top: min(4vh, 30px);
  min-height: 400px;
  width: 100%;
  padding-bottom: min(80px, 10vh);
  margin-top: 80px; /* 调整为导航栏高度+下移距离+间距 */
}

/* 大Logo容器样式 */
.logo-container {
  width: 100%;
  display: flex;
  justify-content: center;
  margin-bottom: min(40px, 5vh);
  padding: 0 20px;
  margin-top: 0;
}

/* 大Logo图片样式 */
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

/* 按钮组容器样式 */
.buttons-container {
  display: flex;
  flex-direction: column;
  gap: min(28px, 3.5vh);
  width: 100%;
  align-items: center;
  padding: 0;
  margin-top: 20px;  /* 从-10px改为20px，下移按钮组 */
  position: relative;
  z-index: 2;
}

/* 按钮基础样式 */
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

  /* 主要按钮样式（启动按钮） */
  &.primary {
    background-color: #4294EC;
    color: #FFFFFF;
  }

  /* 次要按钮样式（其他按钮） */
  &.secondary {
    background-color: #B0DAF3;
    color: #214D89;
  }

  /* 悬停效果 */
    &:hover {
      transform: translateY(-4px);
      box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2);
  }
}

/* 按钮图标样式 */
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

/* 按钮文字样式 */
.button-text {
  text-align: center;
  flex-grow: 1;
  letter-spacing: 0.5px;
  white-space: nowrap;
    }

/* 语言选择器样式 */
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

/* 语言文字样式 */
.lang-text {
  color: #000000;
  font-weight: 700;
  font-size: 18px;
  text-align: center;
  }

/* 语言选项下拉菜单样式 */
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

/* 语言选项样式 */
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

/* ==================== 页面进场动画 ==================== */

/* 定义从下方淡入的动画 */
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

/* 定义从左侧滑入的动画 */
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

/* 定义从右侧滑入的动画 */
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

/* 定义缩放淡入的动画 */
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

/* 将动画应用到各个元素上 */
.fade-in-up {
  animation: fadeIn-Up 0.8s ease-out forwards;
  animation-fill-mode: forwards;
}

/* Logo容器动画 */
.logo-container.fade-in-up {
  animation: scaleIn 1s ease-out 0.2s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 确保Logo悬停效果在动画完成后仍然有效 */
.logo-container.fade-in-up .main-logo:hover {
  transform: scale(1.05) !important;
}

/* 按钮组容器动画 */
.buttons-container.fade-in-up {
  animation: fadeIn-Up 0.8s ease-out 0.4s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 按钮动画 - 错开延迟 */
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

/* 确保悬停效果在动画完成后仍然有效 */
.action-button.fade-in-up:hover {
  transform: translateY(-4px) !important;
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2) !important;
}

/* 确保语言选择器悬停效果在动画完成后仍然有效 */
.option:hover {
  background: rgba(0, 0, 0, 0.1) !important;
  transform: scale(1.02) !important;
}

/* 为背景装饰也添加延迟动画 */
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