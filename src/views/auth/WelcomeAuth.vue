<!--
 * @file WelcomeAuth.vue
 * @description 用户欢迎页面组件
 * @features
 * - 根据登录状态显示不同内容
 * - 多语言支持
 * - 导航栏集成
 * - 路由导航
 -->

<template>
  <div class="welcome-container">
    <!-- 顶部导航栏 -->
    <AuthNavBar />
    
    <!-- 背景装饰 -->
    <div class="background-ellipse fade-in-up"></div>
    <div class="background-ellipse-second fade-in-up"></div>
    <div class="background-ellipse-right fade-in-up"></div>
    <div class="background-ellipse-second-right fade-in-up"></div>
    
    <!-- 主要内容区域 -->
    <div class="welcome-content fade-in-up">
      <!-- Logo展示 -->
      <div class="logo-container fade-in-up">
        <img :src="iconImage" alt="Icon" class="welcome-icon fade-in-up">
        <img :src="logoText" alt="Logo" class="welcome-logo fade-in-up">
      </div>
      
      <!-- 已登录用户欢迎信息 -->
      <div v-if="userStore.isAuthenticated" class="welcome-message fade-in-up">
        <button class="action-button primary fade-in-up" @click="btnContinue">
          {{ $t('welcome.continue') }}
        </button>
      </div>
      <!-- 未登录用户操作按钮 -->
      <div v-else class="auth-buttons fade-in-up">
        <button class="action-button primary fade-in-up" @click="router.push('/login')">
          {{ $t('auth.login.signIn') }}
        </button>
        <button class="action-button secondary fade-in-up" @click="router.push('/register')">
          {{ $t('auth.register.createAccount') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
// 导入所需的依赖
import { useRouter } from 'vue-router'
import { useUserStore } from '../../stores/user'
import AuthNavBar from '../../components/AuthNavBar.vue'
import iconImage from '@images/icon.png'
import logoText from '@images/logo-text1.png'

// 初始化路由和状态管理
const router = useRouter()
const userStore = useUserStore()

// 继续使用按钮点击事件
const btnContinue = async () => {
  try {
    await router.push({ name: 'home' })
  } catch (err) {
    console.error('Navigation error:', err)
  }
}
</script>

<style scoped>
/* 容器样式 */
.welcome-container {
  position: relative;
  min-height: 100vh;
  background-color: #FFFFFF;
  display: flex;
  flex-direction: column;
  overflow: hidden;
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
  z-index: 1;
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
  z-index: 1;
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

/* 主要内容区域样式 */
.welcome-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: clamp(20px, 4vh, 40px);
  position: relative;
  z-index: 2;
  margin-top: clamp(-40px, -5vh, -20px);
}

/* Logo容器样式 */
.logo-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: clamp(30px, 5vh, 50px);
  margin-bottom: clamp(40px, 8vh, 80px);
  width: 100%;
  padding: 0 20px;
  margin-top: -80px;
}

/* Icon样式 */
.welcome-icon {
  width: clamp(120px, 15vw, 200px);
  height: auto;
  margin-bottom: clamp(20px, 3vh, 40px);
  transition: transform 0.8s ease;
}

.welcome-icon:hover {
  transform: scale(1.05) !important;
}

/* Logo样式 */
.welcome-logo {
  width: clamp(600px, 65vw, 1100px);
  height: auto;
  transition: transform 0.8s ease;
}

.welcome-logo:hover {
  transform: scale(1.02) !important;
}

/* 响应式布局调整 */
@media (max-width: 1200px) {
  .welcome-logo {
    width: clamp(500px, 70vw, 800px);
  }
}

@media (max-width: 768px) {
  .welcome-logo {
    width: clamp(400px, 85vw, 600px);
  }
}

@media (max-width: 480px) {
  .welcome-logo {
    width: clamp(280px, 90vw, 400px);
  }
  
  .logo-container {
    margin-top: -40px;
    gap: clamp(20px, 3vh, 30px);
  }
}

/* 欢迎信息样式 */
.welcome-message {
  margin-bottom: clamp(20px, 4vw, 30px);
}

.welcome-message h2 {
  color: #333;
  margin-bottom: clamp(15px, 3vw, 20px);
  font-size: clamp(18px, 4vw, 24px);
}

/* 认证按钮组样式 */
.auth-buttons {
  display: flex;
  gap: clamp(60px, 8vw, 120px);
  justify-content: center;
  width: 100%;
  max-width: min(90vw, 800px);
  padding: 0 20px;
}

/* 按钮基础样式 */
.action-button {
  padding: clamp(10px, 1.8vh, 20px) clamp(25px, 3.5vw, 50px);
  border: none;
  border-radius: 30px;
  font-size: clamp(20px, 1.8vw, 26px);
  font-weight: 700;
  cursor: pointer;
  transition: all 0.8s ease;
  min-width: clamp(160px, 20vw, 200px);
  white-space: nowrap;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #FFFFFF;
  letter-spacing: 2px;
}

/* 主要按钮样式 */
.action-button.primary {
  background-color: #000000;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.15);
}

.action-button.primary:hover {
  background-color: #333333;
  transform: translateY(-2px) !important;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.25) !important;
}

/* 次要按钮样式 */
.action-button.secondary {
  background-color: #000000;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.15);
}

.action-button.secondary:hover {
  background-color: #333333;
  transform: translateY(-2px) !important;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.25) !important;
}

/* 响应式布局调整 */
@media (max-width: 640px) {
  .auth-buttons {
    flex-direction: column;
    align-items: center;
  }

  .action-button {
    width: 100%;
    max-width: 300px;
  }

  .welcome-content {
    margin-top: 0;
  }
}

@media (max-height: 600px) {
  .logo-container {
    gap: 15px;
    margin-bottom: 30px;
  }

  .welcome-icon {
    margin-bottom: 10px;
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

/* 定义缩放淡入的动画 */
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

/* 将动画应用到各个元素上 */
.fade-in-up {
  animation: fadeIn-Up 0.8s ease-out forwards;
  animation-fill-mode: forwards;
}

/* 背景装饰动画 - 错开时间从两侧进入 */
.background-ellipse.fade-in-up {
  animation: slideIn-Left 1.0s ease-out 0.1s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.background-ellipse-second.fade-in-up {
  animation: slideIn-Left 1.0s ease-out 0.2s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.background-ellipse-right.fade-in-up {
  animation: slideIn-Right 1.0s ease-out 0.1s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.background-ellipse-second-right.fade-in-up {
  animation: slideIn-Right 1.0s ease-out 0.2s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 主要内容区域动画 - 缩放淡入 */
.welcome-content.fade-in-up {
  animation: scaleIn 0.8s ease-out 0.3s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* Logo容器动画 - 从下方淡入 */
.logo-container.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.5s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* Logo元素动画 - 错开时间从下方淡入 */
.welcome-icon.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.7s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.welcome-logo.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.9s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 按钮区域动画 - 从下方淡入 */
.welcome-message.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 1.1s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.auth-buttons.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 1.1s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 按钮动画 - 错开时间从下方淡入 */
.action-button.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 1.3s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.action-button.fade-in-up:nth-child(2) {
  animation-delay: 1.4s;
}
</style> 