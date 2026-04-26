<script setup>
import { ref, computed, onMounted } from 'vue';
import { writeTextFile, readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import router from '../router';
import NavBar from '../components/NavBar.vue';
import ReturnButton from '../components/ReturnButton.vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

const appid = ref('');
const apiSecret = ref('');
const apiKey = ref('');
const enableASR = ref(true);

const isASREnabled = computed(() => enableASR.value);

const saveConfig = async () => {
  const config = {
    xfyun: isASREnabled.value
      ? {
          appid: appid.value,
          apiSecret: apiSecret.value,
          apiKey: apiKey.value,
        }
      : null,
    enableASR: enableASR.value,
  };

  try {
    await writeTextFile('config.json', JSON.stringify(config), { dir: BaseDirectory.AppConfig });
    router.push({ name: 'list' });
  } catch (error) {
    console.error('Error saving config:', error);
    // 处理保存错误
  }
};

const loadConfig = async () => {
  try {
    const configContent = await readTextFile('config.json', { dir: BaseDirectory.AppConfig });
    const config = JSON.parse(configContent);
    
    enableASR.value = config.enableASR;
    if (config.xfyun) {
      appid.value = config.xfyun.appid;
      apiSecret.value = config.xfyun.apiSecret;
      apiKey.value = config.xfyun.apiKey;
    }
  } catch (error) {
    console.error('Error loading config:', error);
    // 如果配置文件不存在或读取失败，使用默认值
  }
};

onMounted(() => {
  loadConfig();
});
</script>

<template>
  <div class="container">
    <!-- 导航栏 -->
    <NavBar />
    
    <div class="config-box fade-in-up">
      <h1 class="fade-in-up">{{ t('config.title') }}</h1>
      <form @submit.prevent="saveConfig" class="fade-in-up">
        <div class="form-item checkbox-item fade-in-up">
          <label>
            <input type="checkbox" v-model="enableASR">
            {{ t('config.enableASR') }}
          </label>
        </div>
        <template v-if="isASREnabled">
          <div class="form-item fade-in-up">
            <label for="appid">{{ t('config.appid') }}</label>
            <input id="appid" v-model="appid" required>
          </div>
          <div class="form-item fade-in-up">
            <label for="apiSecret">{{ t('config.apiSecret') }}</label>
            <input id="apiSecret" v-model="apiSecret" required>
          </div>
          <div class="form-item fade-in-up">
            <label for="apiKey">{{ t('config.apiKey') }}</label>
            <input id="apiKey" v-model="apiKey" required>
          </div>
        </template>
        <button type="submit" class="fade-in-up">{{ t('config.saveConfig') }}</button>
      </form>
    </div>
    <ReturnButton class="fade-in-up" />
  </div>
</template>

<style scoped>
.container {
  display: flex;
  background-color: #FFFFFF;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 100vw;
  height: 100vh;
  padding-top: 80px; /* 调整为导航栏高度+下移距离+间距 */
  position: relative;
}

/* 返回按钮固定定位 */
:deep(.return) {
  position: fixed;
  left: 20px;
  bottom: 20px;
  z-index: 1000;
}

.config-box {
  position: relative;
  padding: 30px 40px;
  border-radius: 30px;
  background: #C9E4F4;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  width: 500px;
  z-index: 1;
  box-shadow: 0 8px 32px rgba(33, 77, 137, 0.15);
}

/* 响应式布局调整 */
@media screen and (max-width: 768px) {
  .config-box {
    width: 90%;
    padding: 30px 25px;
    margin: 0 15px;
  }

  h1 {
    font-size: 28px;
    margin-bottom: 40px;  /* 调整响应式下的间距 */
  }

  .form-item {
    margin-bottom: 30px;  /* 调整响应式下的间距 */
  }

  .checkbox-item {
    margin-bottom: 35px;  /* 调整响应式下的间距 */
  }

  input[type="text"],
  input[type="password"],
  input {
    padding: 12px 15px;
    font-size: 1rem;
  }

  button {
    padding: 12px;
    font-size: 1.4rem;  /* 平板端字体大小 */
  }

  label {
    font-size: 1.1rem;
  }
}

@media screen and (max-width: 480px) {
  .config-box {
    width: 95%;
    padding: 25px 20px;
  }

  h1 {
    font-size: 24px;
    margin-bottom: 30px;  /* 调整移动端的间距 */
  }

  .form-item {
    margin-bottom: 25px;  /* 调整移动端的间距 */
  }

  .checkbox-item {
    margin-bottom: 30px;  /* 调整移动端的间距 */
  }

  input[type="text"],
  input[type="password"],
  input {
    padding: 10px 12px;
    font-size: 0.95rem;
  }

  button {
    padding: 10px;
    font-size: 1.3rem;  /* 移动端字体大小 */
  }

  label {
    font-size: 1rem;
  }

  input[type="checkbox"] {
    width: 20px;
    height: 20px;
  }
}

/* 优化输入框阴影效果 */
input:focus {
  outline: none;
  border-color: #214D89;
  box-shadow: 0 0 20px rgba(33, 77, 137, 0.1);
}

/* 优化按钮悬停效果 */
button:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(66, 148, 236, 0.3);
}

h1 {
  text-align: center;
  color: #214D89;
  font-size: 32px;
  margin-bottom: 35px;  /* 减小标题下方间距 */
  font-weight: bold;
  padding-bottom: 15px;  /* 减小标题下边框的内边距 */
  border-bottom: 2px solid #214D89;
}

.form-item {
  margin-bottom: 25px;  /* 减小未勾选状态的间距 */
  position: relative;
}

/* 勾选状态下的间距调整 */
:deep(.form-item:has(+ template[v-if="isASREnabled"])) {
  margin-bottom: 15px;  /* 减小勾选框和下方表单的间距 */
}

template[v-if="isASREnabled"] .form-item {
  margin-bottom: 12px;  /* 减小勾选后表单项之间的间距 */
}

.checkbox-item {
  margin-bottom: 35px;  /* 减小未勾选状态下的间距 */
}

.checkbox-item:has(input:checked) {
  margin-bottom: 15px;  /* 减小勾选状态下的间距 */
}

label {
  display: flex;
  align-items: center;
  color: #214D89;
  margin-bottom: 8px;  /* 减小标签和输入框之间的间距 */
  font-size: 1.2rem;
  font-weight: bold;
  cursor: pointer;
}

input[type="text"],
input[type="password"],
input:not([type="checkbox"]) {
  width: 100%;
  padding: 15px 20px;
  border-radius: 15px;
  border: none;  /* 移除边框 */
  background: #FFFFFF;  /* 白色背景 */
  color: #214D89;
  font-size: 1.1rem;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(33, 77, 137, 0.1);  /* 默认阴影 */
}

input:not([type="checkbox"]):focus {
  outline: none;
  box-shadow: 0 6px 16px rgba(33, 77, 137, 0.15);  /* 聚焦时加深阴影 */
  transform: translateY(-1px);  /* 轻微上浮效果 */
}

/* 响应式布局调整 */
@media screen and (max-width: 768px) {
  template[v-if="isASREnabled"] .form-item {
    margin-bottom: 10px;  /* 平板端勾选后的间距 */
  }
  
  .checkbox-item:has(input:checked) {
    margin-bottom: 12px;
  }

  h1 {
    margin-bottom: 30px;
  }
}

@media screen and (max-width: 480px) {
  template[v-if="isASREnabled"] .form-item {
    margin-bottom: 8px;  /* 移动端勾选后的间距 */
  }
  
  .checkbox-item:has(input:checked) {
    margin-bottom: 10px;
  }

  h1 {
    margin-bottom: 25px;
  }

  label {
    margin-bottom: 6px;
  }

  input[type="text"],
  input[type="password"],
  input:not([type="checkbox"]) {
    padding: 12px 15px;
    box-shadow: 0 3px 10px rgba(33, 77, 137, 0.1);  /* 移动端阴影稍微减小 */
  }
}

/* 优化多选框样式 */
input[type="checkbox"] {
  -webkit-appearance: none;
  appearance: none;
  width: 22px;  /* 增加多选框大小 */
  height: 22px;  /* 增加多选框大小 */
  border: 2px solid #214D89;
  border-radius: 5px;
  margin-right: 12px;
  position: relative;
  cursor: pointer;
  transition: all 0.2s ease;
  padding: 0;
}

input[type="checkbox"]:checked {
  background-color: #214D89;
  border-color: #214D89;
}

input[type="checkbox"]:checked::after {
  content: "✓";
  position: absolute;
  color: white;
  font-size: 14px;  /* 增加勾选标记大小 */
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
}

button {
  width: 100%;
  padding: 15px;  /* 恢复原有内边距 */
  border: none;
  border-radius: 15px;
  background-color: #4294EC;
  color: #FFFFFF;
  font-size: 1.5rem;  /* 只增大字体大小 */
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
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

/* 将动画应用到各个元素上 */
.fade-in-up {
  animation: fadeIn-Up 0.8s ease-out forwards;
  animation-fill-mode: forwards;
}

/* 确保动画选择器优先级足够高 */
.container .config-box.fade-in-up {
  animation: scaleIn 0.6s ease-out 0.1s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.container h1.fade-in-up {
  animation: fadeIn-Up 0.5s ease-out 0.2s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.container form.fade-in-up {
  animation: fadeIn-Up 0.5s ease-out 0.3s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.container .checkbox-item.fade-in-up {
  animation: fadeIn-Up 0.5s ease-out 0.4s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.container .form-item.fade-in-up {
  animation: slideIn-Left 0.5s ease-out 0.5s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.container .form-item.fade-in-up:nth-child(2) {
  animation-delay: 0.6s;
}

.container .form-item.fade-in-up:nth-child(3) {
  animation-delay: 0.7s;
}

.container button.fade-in-up {
  animation: fadeIn-Up 0.5s ease-out 0.8s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.container :deep(.return.fade-in-up) {
  animation: fadeIn-Up 0.5s ease-out 0.9s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}
</style>
