<template>
  <div class="return" @click="btnReturn">
    <img src="/src-tauri/icons/icon5.png" class="return-icon" alt="Return" />
    <span class="return-text">{{ $t('return') }}</span>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'

const router = useRouter()
const { t } = useI18n()

const props = defineProps({
  beforeReturn: {
    type: Function,
    default: null
  }
})

const btnReturn = async () => {
  if (props.beforeReturn) {
    await props.beforeReturn()
  }
  router.back()
}
</script>

<style scoped>
.return {
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
  margin-left: -4px;  /* 微调图标位置 */
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
</style> 