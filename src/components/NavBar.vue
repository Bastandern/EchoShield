<template>
  <div class="nav-wrapper">
    <div class="header">
      <div class="logo">
        <img src="/images/logo.png" alt="AudioShield" class="small-logo">
      </div>
      <div class="nav-right">
        <div class="user-menu" v-if="userStore.isAuthenticated" @click="isUserMenuOpen = !isUserMenuOpen">
          <span class="username">{{ userStore.user.username }}</span>
          <div class="menu-options" v-if="isUserMenuOpen">
            <div class="menu-option" @click="handleLogout">{{ $t('nav.logout') }}</div>
          </div>
        </div>
        <div class="lang-select" @click="isLangOpen = !isLangOpen">
          <span class="lang-text">{{ locale === 'zh' ? '语言' : 'Language' }}</span>
          <div class="options" v-if="isLangOpen">
            <div class="option" @click="changeLanguage('zh')">中文</div>
            <div class="option" @click="changeLanguage('en')">English</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { useUserStore } from '../stores/user'
import Cookies from 'js-cookie'

const router = useRouter()
const { locale } = useI18n()
const userStore = useUserStore()

const isLangOpen = ref(false)
const isUserMenuOpen = ref(false)

const changeLanguage = (lang) => {
  locale.value = lang
  Cookies.set('language', lang)
  isLangOpen.value = false
}

const handleLogout = () => {
  userStore.clearUser()
  router.push('/login')
  isUserMenuOpen.value = false
}
</script>

<style scoped>
/* 导航栏包装容器 */
.nav-wrapper {
  position: fixed;
  top: 10px; /* 从顶部下移10px */
  left: 0;
  right: 0;
  z-index: 1000;
  background-color: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  padding: 0 20px; /* 添加左右内边距 */
  pointer-events: none; /* 添加这一行，让导航栏不阻挡点击事件 */
}

/* 顶部导航栏样式 */
.header {
  pointer-events: auto; /* 让header内的元素可以点击 */
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  padding: 10px 0;
  min-height: 60px;
  border-bottom: 1px solid #E7E7E7;
  margin-bottom: 0;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 15px; /* 添加圆角 */
  z-index: 100;
}

/* Logo样式 */
.logo {
  display: flex;
  align-items: center;
  margin-top: -10px;
  margin-left: 40px;  /* 增加左边距使Logo右移 */
}

.small-logo {
  height: 36px;
  width: auto;
  object-fit: contain;
}

/* 导航栏右侧样式 */
.nav-right {
  display: flex;
  align-items: center;
  gap: 20px;
  margin-top: -10px;
  margin-right: 40px;  /* 增加右边距使内容左移 */
}

/* 语言选择器和用户菜单样式 */
.lang-select, .user-menu {
  position: relative;
  cursor: pointer;
  z-index: 100;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.lang-text, .username {
  color: #000000;
  font-weight: 700;
  font-size: 18px;
  text-align: center;
}

/* 下拉菜单样式 */
.menu-options {
  position: absolute;
  top: 100%;
  right: 20px;
  margin-top: 8px;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  min-width: 90px;
  transform: translateX(25%);
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

.option, .menu-option {
  padding: 8px 16px;
  cursor: pointer;
  white-space: nowrap;
  font-size: 16px;
  color: #000000;
  font-weight: 600;
  text-align: center;
}

.option:hover, .menu-option:hover {
  background-color: #f5f5f5;
}

.menu-option {
  color: #dc3545;
}
</style> 