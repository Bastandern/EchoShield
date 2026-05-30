<script setup>
import { ref } from 'vue';

import axios from "axios";
import router from '../router'
import { exists, createDir, writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

import { useI18n } from 'vue-i18n';
import NavBar from '../components/NavBar.vue';
import ReturnButton from '../components/ReturnButton.vue';

const { t } = useI18n();

const displayMessage = ref(t('async.displaySync'))

const baseURL = import.meta.env.VITE_BASE_URL;
const timeout = 10000;

// 将 UAP 写入文件
const writeUAP = async (data) => {
  if (!await exists('uap', { dir: BaseDirectory.AppData })) {
    console.log('create dir')
    await createDir('uap', { dir: BaseDirectory.AppData, recursive: true });
  }
  writeTextFile('uap/uap.ar', data, {
    dir: BaseDirectory.AppData
  });
  setTimeout(() => {
    router.push("/async-upload");
  }, 1000);
}

// 从远程获取 UAP 信息
const initUAP = async () => {
  const perturbationPath = import.meta.env.VITE_PERTURBATION_PATH || 'uap.ar';
  
  axios.create({
    baseURL: baseURL,
    timeout: timeout,
  }).get(perturbationPath)
    .then((res) => {
      if (res.status === 200) {
        writeUAP(res.data)
      } else {
        setTimeout(() => {
          router.push("/async-upload");
        }, 1000);
      }
    })
    .catch((_err) => {
      setTimeout(() => {
        router.push("/async-upload");
      }, 1000);
    });
}

initUAP()

</script>

<template>
  <div class="container">
    <!-- 导航栏 -->
    <NavBar />
    
    <div class="title">
      <div class="content">
        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 256 256">
          <path fill="currentColor"
            d="M244 56v48a12 12 0 0 1-12 12h-48a12 12 0 1 1 0-24h17.1l-19-17.38c-.13-.12-.26-.24-.38-.37A76 76 0 1 0 127 204h1a75.53 75.53 0 0 0 52.15-20.72a12 12 0 0 1 16.49 17.45A99.45 99.45 0 0 1 128 228h-1.37a100 100 0 1 1 71.88-170.94L220 76.72V56a12 12 0 0 1 24 0Z" />
        </svg>
        <span>{{ displayMessage }}</span>
      </div>
    </div>
    <ReturnButton />
  </div>
</template>

<style scoped>
.container {
  display: flex;
  background-color: #FFFFFF;
  width: 100vw;
  height: 100vh;
  align-items: center;
  justify-content: center;
  padding-top: 40px;
  position: relative;
}

:deep(.return) {
  position: fixed;
  left: 20px;
  bottom: 20px;
  z-index: 1000;
}

@keyframes loading {
  0% {
    transform: rotate(0);
  }

  100% {
    transform: rotate(360deg);
  }
}

.title {
  position: relative;
  padding: 30px 60px;
  border-radius: 30px;
  background: #C9E4F4;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  box-shadow: 0 8px 32px rgba(33, 77, 137, 0.15);
  z-index: 1;

  .content {
    display: flex;
    align-items: center;
    justify-content: center;
    color: #214D89;
    font-size: 34px;
    font-weight: bold;

    svg {
      animation: loading 1s infinite linear;
      margin-right: 12px;
      width: 36px;
      height: 36px;
    }
  }
}
</style>
