<script setup>
import ReturnButton from '../components/ReturnButton.vue';

</script>

<template>
  <div class="container">
    <div class="title">
      <div class="text">404</div>
      <div class="subtitle">{{ $t('notfound.notfound') }}</div>
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
  flex-direction: column;
  justify-content: center;
  align-items: center;
  position: relative;
}

/* 返回按钮固定定位 */
:deep(.return) {
  position: fixed;
  left: 20px;
  bottom: 20px;
  z-index: 1000;
}

.title {
  position: relative;
  padding: 60px 120px;
  border-radius: 40px;
  background: #C9E4F4;
  backdrop-filter: blur(10px);
  text-align: center;
  box-shadow: 0 8px 32px rgba(66, 148, 236, 0.15);

  .text {
    color: #4294EC;
    font-size: 120px;
    font-weight: bold;
    line-height: 1.2;
  }

  .subtitle {
    color: #4294EC;
    font-size: 36px;
    margin-top: 20px;
    font-weight: 500;
  }


}

.return {
  position: fixed;
  left: 20px;
  bottom: 20px;
  padding: 8px 20px;
  border-radius: 12px;
  background: #C9E4F4;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 8px;
  z-index: 1000;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 20px #ec8c8922;
  }

  svg {
    fill: currentColor;
  }
}
</style>
