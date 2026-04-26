<!--
 * @file RegisterView.vue
 * @description 用户注册页面组件
 * @features
 * - 用户注册表单
 * - 密码确认验证
 * - 多语言支持
 * - 导航栏集成
 * - 错误处理和加载状态
 * - 路由导航
 -->

<template>
  <div class="register-container">
    <!-- 顶部导航栏部分 -->
    <AuthNavBar />

    <!-- 背景装饰 -->
    <div class="background-ellipse"></div>
    <div class="background-ellipse-second"></div>
    <div class="background-ellipse-right"></div>
    <div class="background-ellipse-second-right"></div>

    <!-- 注册表单部分 -->
    <div class="register-content">
      <div class="register-box fade-in-up">
        <h2 class="register-title">{{ $t('auth.register.title') }}</h2>
        <!-- 注册表单 -->
        <form class="register-form" @submit.prevent="handleRegister">
          <!-- 用户名输入框 -->
          <div class="form-group">
            <label for="username">{{ $t('auth.register.username') }}</label>
            <input 
              type="text" 
              id="username" 
              v-model="username" 
              required
              :placeholder="$t('auth.register.usernamePlaceholder')"
            >
          </div>
          <!-- 密码输入框 -->
          <div class="form-group">
            <label for="password">{{ $t('auth.register.password') }}</label>
            <input 
              type="password" 
              id="password" 
              v-model="password" 
              required
              :placeholder="$t('auth.register.passwordPlaceholder')"
            >
          </div>
          <!-- 确认密码输入框 -->
          <div class="form-group">
            <label for="confirmPassword">{{ $t('auth.register.confirmPassword') }}</label>
            <input 
              type="password" 
              id="confirmPassword" 
              v-model="confirmPassword" 
              required
              :placeholder="$t('auth.register.confirmPasswordPlaceholder')"
            >
          </div>
          <!-- 错误信息显示 -->
          <div class="error-message" v-if="error">{{ error }}</div>
          <!-- 注册按钮 -->
          <button type="submit" class="register-button" :disabled="loading">
            {{ loading ? $t('auth.register.creatingAccount') : $t('auth.register.createAccount') }}
          </button>
          <!-- 登录链接 -->
          <div class="form-footer">
            <span>{{ $t('auth.register.hasAccount') }}</span>
            <a @click="router.push('/login')" class="login-link">{{ $t('auth.register.signIn') }}</a>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup>
// 导入所需的依赖
import { ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/tauri'
import { useUserStore } from '../../stores/user'
import AuthNavBar from '../../components/AuthNavBar.vue'

// 初始化路由、国际化和状态管理
const router = useRouter()
const { t } = useI18n()
const userStore = useUserStore()

// 定义响应式状态
const username = ref('') // 用户名输入
const password = ref('') // 密码输入
const confirmPassword = ref('') // 确认密码输入
const error = ref('') // 错误信息
const loading = ref(false) // 加载状态

watch([password, confirmPassword], ([newPassword, newConfirmPassword]) => {
  if (newPassword && newConfirmPassword && newPassword !== newConfirmPassword) {
    error.value = t('auth.register.passwordsNotMatch')
  } else {
    error.value = ''
  }
})

// 注册表单提交处理函数
const handleRegister = async () => {
  try {
    // 验证密码是否匹配
  if (password.value !== confirmPassword.value) {
    error.value = t('auth.register.passwordsNotMatch')
    return
  }

    loading.value = true
    error.value = ''
    
    // 调用后端注册 API
    const user = await invoke('register', {
      username: username.value,
      password: password.value
    })
    
    if (!user) {
      error.value = t('auth.register.registrationFailed')
      return
    }

    // 保存用户信息到 store
    userStore.setUser(user)
    
    // 注册成功，跳转到首页
    router.push('/home')
  } catch (err) {
    console.error('Registration error:', err)
    error.value = t('auth.register.registrationFailed')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
/* 容器样式 */
.register-container {
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

/* 注册内容区域样式 */
.register-content {
  position: relative;
  z-index: 2;
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: clamp(15px, 3vw, 30px);
  min-height: 100vh;
  padding-bottom: 15vh;
}

/* 注册框样式 */
.register-box {
  position: relative;
  z-index: 2;
  width: 100%;
  max-width: clamp(350px, 85vw, 500px);
  padding: clamp(14px, 2.2vw, 22px);
  border-radius: clamp(20px, 3vw, 25px);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.15);
  background-color: #C9E4F4;
  margin-top: clamp(-50px, -8vh, -30px);
}

/* 标题样式 */
.register-title {
  text-align: center;
  color: #214D89;
  margin-bottom: clamp(14px, 2.2vw, 18px);
  font-size: clamp(28px, 5vw, 40px);
  position: relative;
  padding-bottom: clamp(6px, 1vw, 8px);
  font-weight: 600;
}

.register-title::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 12%;
  right: 12%;
  height: clamp(2px, 0.4vw, 3px);
  background-color: #214D89;
}

/* 表单样式 */
.register-form {
  display: flex;
  flex-direction: column;
  gap: clamp(10px, 1.8vw, 14px);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: clamp(3px, 0.9vw, 5px);
}

/* 输入框样式 */
.form-group label {
  font-size: clamp(18px, 2.5vw, 22px);
  color: #214D89;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.form-group input {
  padding: clamp(7px, 1.3vw, 11px);
  border: 1px solid #ddd;
  border-radius: clamp(10px, 1.5vw, 12px);
  font-size: clamp(14px, 1.8vw, 16px);
  width: 100%;
  transition: all 0.3s ease;
}

.form-group input:focus {
  outline: none;
  border-color: #4294EC;
}

/* 注册按钮样式 */
.register-button {
  background-color: #4294EC;
  color: white;
  padding: clamp(7px, 1.3vw, 11px);
  border: none;
  border-radius: clamp(10px, 1.5vw, 12px);
  font-size: clamp(18px, 2.2vw, 22px);
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  margin-top: clamp(3px, 0.9vw, 7px);
  width: 100%;
  letter-spacing: 1px;
}

.register-button:hover {
  background-color: #3276b1;
  transform: translateY(-2px);
}

.register-button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

/* 错误信息样式 */
.error-message {
  color: #dc3545;
  font-size: 14px;
  text-align: center;
}

/* 表单底部样式 */
.form-footer {
  text-align: center;
  font-size: clamp(14px, 2.2vw, 20px);
  color: #666;
  margin-top: clamp(5px, 1.1vw, 9px);
  letter-spacing: 0.5px;
}

/* 登录链接样式 */
.login-link {
  color: #214D89;
  margin-left: 8px;
  cursor: pointer;
  text-decoration: none;
  font-weight: 600;
}

.login-link:hover {
  text-decoration: underline;
  opacity: 0.9;
}

/* 响应式布局调整 */
@media (min-width: 1200px) {
  .register-box {
    max-width: 500px;
    padding: 50px;
  }
  
  .register-title {
    font-size: 46px;
  }
  
  .form-group label {
    font-size: 24px;
  }
  
  .register-button {
    font-size: 24px;
  }
}

@media (max-width: 1024px) {
  .register-box {
    max-width: 80vw;
  }
}

@media (max-width: 768px) {
  .register-content {
    padding: clamp(10px, 2vw, 15px);
  }
  
  .register-box {
    max-width: 85vw;
    padding: clamp(20px, 3vw, 30px);
  }
  
  .register-title::after {
    left: 10%;
    right: 10%;
  }
}

@media (max-width: 480px) {
  .register-content {
    padding: 10px;
    align-items: flex-start;
    padding-top: clamp(10px, 3vh, 20px);
    padding-bottom: 10vh;
  }
  
  .register-box {
    max-width: 92vw;
    padding: 14px;
    margin-top: 0;
  }
  
  .register-title {
    font-size: clamp(22px, 5vw, 26px);
    margin-bottom: 14px;
    padding-bottom: 5px;
  }
  
  .register-title::after {
    left: 8%;
    right: 8%;
  }
  
  .register-form {
    gap: 10px;
  }
  
  .form-group {
    gap: 3px;
  }
  
  .form-group input {
    padding: 7px;
  }
  
  .form-group label {
    font-size: 16px;
  }
  
  .register-button {
    padding: 9px;
    font-size: 18px;
    margin-top: 4px;
  }
  
  .form-footer {
    font-size: 14px;
    margin-top: 5px;
  }
}

@media (max-width: 360px) {
  .register-box {
    max-width: 96vw;
    padding: 15px;
  }
  
  .register-title {
    font-size: 22px;
    margin-bottom: 15px;
  }
  
  .form-group label {
    font-size: 16px;
  }
  
  .register-button {
    font-size: 16px;
  }
  
  .form-footer {
    font-size: 14px;
  }
}

@media (max-height: 600px) {
  .register-content {
    padding-top: 10px;
    padding-bottom: 4vh;
  }
  
  .register-box {
    margin-top: 0;
    padding: 12px;
  }
  
  .register-title {
    margin-bottom: 12px;
    font-size: 24px;
  }
  
  .register-form {
    gap: 8px;
  }
  
  .form-group {
    gap: 3px;
  }
  
  .form-group input {
    padding: 7px;
  }
  
  .register-button {
    padding: 7px;
    margin-top: 3px;
  }
  
  .form-footer {
    margin-top: 5px;
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

/* 将动画应用到注册框上 */
.fade-in-up {
  animation: fadeIn-Up 0.8s ease-out forwards;
  /* 确保动画完成后保持最终状态 */
  animation-fill-mode: forwards;
}

/* 为背景装饰也添加延迟动画 */
.background-ellipse {
  animation: fadeIn-Up 1s ease-out 0.2s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.background-ellipse-second {
  animation: fadeIn-Up 1s ease-out 0.4s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.background-ellipse-right {
  animation: fadeIn-Up 1s ease-out 0.3s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.background-ellipse-second-right {
  animation: fadeIn-Up 1s ease-out 0.5s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}
</style> 