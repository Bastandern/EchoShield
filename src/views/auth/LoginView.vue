<template>
  <div class="login-container">
    <AuthNavBar />

    <div class="background-ellipse"></div>
    <div class="background-ellipse-second"></div>
    <div class="background-ellipse-right"></div>
    <div class="background-ellipse-second-right"></div>

    <div class="login-content">
      <div class="login-box fade-in-up">
        <h2 class="login-title">{{ $t('auth.login.title') }}</h2>
        <form class="login-form" @submit.prevent="handleLogin">
          <div class="form-group">
            <label for="username">{{ $t('auth.login.username') }}</label>
            <input
              type="text"
              id="username"
              v-model="username"
              required
              :placeholder="$t('auth.login.usernamePlaceholder')"
            >
          </div>
          <div class="form-group">
            <label for="password">{{ $t('auth.login.password') }}</label>
            <input
              type="password"
              id="password"
              v-model="password"
              required
              :placeholder="$t('auth.login.passwordPlaceholder')"
            >
          </div>
          <div class="error-message" v-if="error">{{ error }}</div>
          <button type="submit" class="login-button" :disabled="loading">
            {{ loading ? $t('auth.login.signingIn') : $t('auth.login.signIn') }}
          </button>
          <div class="form-footer">
            <span>{{ $t('auth.login.noAccount') }}</span>
            <a @click="router.push('/register')" class="register-link">{{ $t('auth.login.signUp') }}</a>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/tauri'
import { useUserStore } from '../../stores/user'
import AuthNavBar from '../../components/AuthNavBar.vue'

const router = useRouter()
const { t } = useI18n()
const userStore = useUserStore()

const username = ref('')
const password = ref('')
const error = ref('')
const loading = ref(false)

const handleLogin = async () => {
  try {
    loading.value = true
    error.value = ''

    const user = await invoke('login', {
      username: username.value,
      password: password.value
    })

    if (!user) {
      error.value = t('auth.login.loginFailed')
      return
    }

    userStore.setUser(user)
    router.push('/home')
  } catch (err) {
    console.error('Login error:', err)
    error.value = t('auth.login.loginFailed')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-container {
  position: relative;
  min-height: 100vh;
  background-color: #FFFFFF;
  display: flex;
  flex-direction: column;
  overflow: hidden;
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

.login-content {
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

.login-box {
  position: relative;
  z-index: 2;
  width: 100%;
  max-width: clamp(350px, 85vw, 500px);
  padding: clamp(20px, 3vw, 35px);
  border-radius: clamp(20px, 3vw, 25px);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.15);
  background-color: #C9E4F4;
  margin-top: clamp(-50px, -8vh, -30px);
}

.login-logo {
  display: none;
}

.login-title {
  text-align: center;
  color: #214D89;
  margin-bottom: clamp(16px, 2.5vw, 22px);
  font-size: clamp(32px, 6vw, 48px);
  position: relative;
  padding-bottom: clamp(8px, 1.2vw, 10px);
  font-weight: 600;
}

.login-title::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 12%;
  right: 12%;
  height: clamp(2px, 0.4vw, 3px);
  background-color: #214D89;
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: clamp(15px, 2.5vw, 25px);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: clamp(6px, 1.2vw, 10px);
}

.form-group label {
  font-size: clamp(18px, 3vw, 24px);
  color: #214D89;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.form-group input {
  padding: clamp(12px, 2vw, 16px);
  border: 1px solid #ddd;
  border-radius: clamp(12px, 2vw, 15px);
  font-size: clamp(16px, 2vw, 18px);
  width: 100%;
  transition: all 0.3s ease;
}

.form-group input:focus {
  outline: none;
  border-color: #4294EC;
}

.login-button {
  background-color: #4294EC;
  color: white;
  padding: clamp(10px, 2vw, 16px);
  border: none;
  border-radius: clamp(12px, 2vw, 15px);
  font-size: clamp(18px, 2.5vw, 24px);
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  margin-top: clamp(5px, 1.5vw, 10px);
  width: 100%;
  letter-spacing: 1px;
}

.login-button:hover {
  background-color: #3276b1;
  transform: translateY(-2px);
}

.login-button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.error-message {
  color: #dc3545;
  font-size: 14px;
  text-align: center;
}

.form-footer {
  text-align: center;
  font-size: clamp(16px, 2.5vw, 22px);
  color: #666;
  margin-top: clamp(8px, 1.5vw, 12px);
  letter-spacing: 0.5px;
}

.register-link {
  color: #214D89;
  margin-left: 8px;
  cursor: pointer;
  text-decoration: none;
  font-weight: 600;
}

.register-link:hover {
  text-decoration: underline;
  opacity: 0.9;
}

@media (min-width: 1200px) {
  .login-box {
    max-width: 500px;
    padding: 50px;
  }

  .login-title {
    font-size: 46px;
  }

  .form-group label {
    font-size: 24px;
  }

  .login-button {
    font-size: 24px;
  }
}

@media (max-width: 1024px) {
  .login-box {
    max-width: 80vw;
  }
}

@media (max-width: 768px) {
  .login-content {
    padding: clamp(10px, 2vw, 15px);
  }

  .login-box {
    max-width: 85vw;
  }

  .login-title::after {
    left: 10%;
    right: 10%;
  }
}

@media (max-width: 480px) {
  .login-content {
    padding: 10px;
    align-items: flex-start;
    padding-top: clamp(10px, 3vh, 20px);
    padding-bottom: 10vh;
  }

  .login-box {
    max-width: 92vw;
    padding: 20px;
    margin-top: 0;
  }

  .login-title {
    font-size: clamp(26px, 6vw, 32px);
    margin-bottom: 16px;
    padding-bottom: 6px;
  }

  .login-title::after {
    left: 8%;
    right: 8%;
  }

  .form-group input {
    font-size: 16px;
    padding: 12px;
  }

  .form-group label {
    font-size: clamp(16px, 5vw, 20px);
  }

  .login-button {
    font-size: clamp(18px, 5vw, 20px);
    padding: 12px;
  }

  .form-footer {
    font-size: clamp(14px, 4vw, 18px);
  }
}

@media (max-width: 360px) {
  .login-box {
    max-width: 96vw;
    padding: 15px;
  }

  .login-title {
    font-size: 24px;
    margin-bottom: 15px;
  }

  .form-group label {
    font-size: 16px;
  }

  .login-button {
    font-size: 16px;
  }

  .form-footer {
    font-size: 14px;
  }
}

@media (max-height: 600px) {
  .login-content {
    padding-top: 10px;
    padding-bottom: 5vh;
  }

  .login-box {
    margin-top: 0;
    padding-top: 15px;
    padding-bottom: 15px;
  }

  .login-title {
    margin-bottom: 15px;
  }

  .login-form {
    gap: 12px;
  }

  .form-group {
    gap: 6px;
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

.fade-in-up {
  animation: fadeIn-Up 0.8s ease-out forwards;
  animation-fill-mode: forwards;
}

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
