<script setup>
import { ref, computed } from 'vue';
import { writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import { useI18n } from 'vue-i18n';
import NavBar from '../components/NavBar.vue';
import router from '../router';

const { t } = useI18n();

const dragActive = ref(false);
const uploadedFileName = ref('');
const uploadStatus = ref('default'); // 'default', 'success', 'error'

// 使用计算属性来保持响应性
const uploadMessage = computed(() => {
  switch (uploadStatus.value) {
    case 'success':
      return t('async.uploadSuccess');
    case 'error':
      return t('async.uploadError');
    default:
      return t('async.uploadMessage');
  }
});

const handleFileUpload = async (event) => {
  const file = event.target.files?.[0] || event.dataTransfer?.files?.[0];
  if (!file) return;

  try {
    const reader = new FileReader();
    reader.onload = async (e) => {
      const fileContent = e.target.result;
      // 将文件内容保存到应用程序数据目录
      await writeTextFile('uap/uap.ar', fileContent, { dir: BaseDirectory.AppData });
      uploadedFileName.value = file.name;
      uploadStatus.value = 'success';
    };
    reader.readAsText(file);
  } catch (error) {
    uploadStatus.value = 'error';
  }
};

const handleDragEnter = () => {
  dragActive.value = true;
};

const handleDragLeave = () => {
  dragActive.value = false;
};

const handleDragOver = (event) => {
  event.preventDefault();
};

const handleDrop = (event) => {
  event.preventDefault();
  dragActive.value = false;
  handleFileUpload(event);
};

// 返回首页
const btnReturn = () => {
  router.push({ name: 'home' });
};
</script>

<template>
  <div class="container">
    <!-- 导航栏 -->
    <NavBar />
    
    <div class="upload-title fade-in-up">
      <p class="upload-message fade-in-up">{{ uploadMessage }}</p>
      <div class="dropzone fade-in-up" :class="{ active: dragActive }" @dragenter="handleDragEnter" @dragleave="handleDragLeave"
        @dragover="handleDragOver" @drop="handleDrop" @click="$refs.fileInput.click()">
        <p v-if="!uploadedFileName">{{ $t('async.dropzone') }}</p>
        <p v-else>{{ $t('async.dropzoneAlready') }} {{ uploadedFileName }}</p>
        <input type="file" ref="fileInput" class="file-input" @change="handleFileUpload" style="display: none" />
      </div>
    </div>
    <div class="return fade-in-up" @click="btnReturn">
      <img src="/src-tauri/icons/icon5.png" class="return-icon" alt="Return" />
      <span class="return-text">{{ $t('return') }}</span>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  background-color: #FFFFFF;
  position: relative;
  padding-top: 80px; /* 为导航栏留出空间 */
}

.return {
  position: absolute;
  left: 20px;
  bottom: 20px;
  padding: 8px 20px;
  border-radius: 12px;
  background-color: #C9E4F4;
  color: #214D89;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 8px;
  user-select: none;
  min-width: 106px;
  justify-content: flex-start;
}

.return-icon {
  width: 24px;
  height: 24px;
  object-fit: contain;
}

.return-text {
  font-size: 16px;
  color: #214D89;
  flex: 1;
  text-align: left;
}

.return:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(201, 228, 244, 0.4);
}

.upload-title {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 40px;
  border-radius: 20px;
  background: #C9E4F4;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  width: 600px; /* 增加容器宽度 */
  box-shadow: 0 8px 32px rgba(33, 77, 137, 0.15); /* 添加阴影效果，使用相同的主题色 */
}

.upload-message {
  font-size: 32px;
  color: #214D89;
  font-weight: bold;
  margin-bottom: 30px;
  text-align: center;
}

.dropzone {
  width: 500px;
  height: 250px;
  border: 2px dashed #214D89;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #3687F4;
  font-size: 20px;
  transition: all 0.3s ease;
  cursor: pointer;
  background: rgba(255, 255, 255, 0.5); /* 添加半透明白色背景 */
}

.dropzone.active {
  background-color: rgba(236, 140, 137, 0.2);
  box-shadow: 0 4px 20px rgba(236, 140, 137, 0.2);
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

/* 将动画应用到各个元素上 */
.fade-in-up {
  animation: fadeIn-Up 0.8s ease-out forwards;
  animation-fill-mode: forwards;
}

/* 上传标题容器动画 - 缩放淡入 */
.upload-title.fade-in-up {
  animation: scaleIn 0.8s ease-out 0.2s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 上传消息动画 - 从下方淡入 */
.upload-message.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.4s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 拖拽区域动画 - 从下方淡入 */
.dropzone.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.6s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 返回按钮动画 - 从下方淡入 */
.return.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.8s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}
</style>
