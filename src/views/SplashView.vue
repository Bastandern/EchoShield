<template>
  <div class="splash-container" :class="{ 'loaded': isLoaded }">
    <div class="background-animation">
      <div class="gradient-bg"></div>
      <div class="texture-layer"></div>
    </div>
    
    <div class="content">
      <div class="logo-container">
        <h1 class="logo-text">
          <span class="letter" v-for="(letter, index) in 'EchoShield'" :key="index" 
                :style="{ animationDelay: `${index * 0.1}s` }">
            {{ letter }}
          </span>
        </h1>
        <div class="subtitle">Adversarial Speech Privacy Protection System</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '../stores/user'

const router = useRouter()
const userStore = useUserStore()
const isLoaded = ref(false)

onMounted(() => {
  // 延迟显示内容，避免白色闪烁
  setTimeout(() => {
    isLoaded.value = true
  }, 100)
  
  // 增加延迟时间，确保exe环境下的稳定性
  setTimeout(() => {
    if (userStore.isAuthenticated) {
      router.push({ name: 'welcome' })
    } else {
      router.push({ name: 'welcome' })
    }
  }, 3000)
})
</script>

<style scoped>
.splash-container {
  width: 100vw;
  height: 100vh;
  position: relative;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, 
    #da4fec 0%,
    #9239f0 30%,
    #4515b4 50%,
    #2b1dab 70%,
    #1552bc 90%,
    #3e7be4 100%);
  opacity: 0;
  transition: opacity 0.5s ease-in-out;
}

.splash-container.loaded {
  opacity: 1;
}

.background-animation {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1;
  opacity: 0;
  transition: opacity 0.8s ease-in-out 0.1s;
}

.splash-container.loaded .background-animation {
  opacity: 1;
}

.gradient-bg {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, 
    #da4fec 0%,
    #9239f0 30%,
    #4515b4 50%,
    #2b1dab 70%,
    #1552bc 90%,
    #3e7be4 100%);
  animation: gradientShift 12s ease-in-out infinite;
  will-change: background;
}

@keyframes gradientShift {
  0%, 100% {
    background: linear-gradient(135deg, 
    #da4fec 0%,
    #9239f0 30%,
    #4515b4 50%,
    #2b1dab 70%,
    #1552bc 90%,
    #3e7be4 100%);
  }
  50% {
    background: linear-gradient(135deg, 
    #dd2af5 0%,
    #8821f5 30%,
    #3c08b5 50%,
    #2011ac 70%,
    #0b4ec3 90%,
    #2a71ec 100%);
  }
}

.texture-layer {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1;
  background-image: 
    linear-gradient(
      90deg,
      transparent 0%,
      transparent calc(50% - 1px),
      rgba(255, 255, 255, 0.08) calc(50% - 1px),
      rgba(255, 255, 255, 0.08) calc(50% + 1px),
      transparent calc(50% + 1px),
      transparent 100%
    ),
    linear-gradient(
      0deg,
      transparent 0%,
      transparent calc(50% - 1px),
      rgba(255, 255, 255, 0.08) calc(50% - 1px),
      rgba(255, 255, 255, 0.08) calc(50% + 1px),
      transparent calc(50% + 1px),
      transparent 100%
    ),
    repeating-linear-gradient(
      45deg,
      transparent 0%,
      transparent 95%,
      rgba(255, 255, 255, 0.04) 95%,
      rgba(255, 255, 255, 0.04) 100%
    ),
    radial-gradient(
      circle at 20% 80%,
      rgba(255, 255, 255, 0.06) 0%,
      transparent 40%
    ),
    radial-gradient(
      circle at 80% 20%,
      rgba(255, 255, 255, 0.06) 0%,
      transparent 40%
    ),
    radial-gradient(
      circle at 40% 40%,
      rgba(255, 255, 255, 0.05) 0%,
      transparent 40%
    ),
    radial-gradient(
      circle at 60% 60%,
      rgba(255, 255, 255, 0.04) 0%,
      transparent 35%
    ),
    radial-gradient(
      circle at 10% 30%,
      rgba(255, 255, 255, 0.05) 0%,
      transparent 45%
    );
  mix-blend-mode: soft-light;
  animation: textureFloat 30s ease-in-out infinite;
  pointer-events: none;
  will-change: transform, opacity;
}

@keyframes textureFloat {
  0%, 100% {
    transform: translate(0, 0) rotate(0deg);
    opacity: 0.7;
  }
  25% {
    transform: translate(1px, -1px) rotate(0.3deg);
    opacity: 0.8;
  }
  50% {
    transform: translate(2px, 1px) rotate(0.6deg);
    opacity: 0.9;
  }
  75% {
    transform: translate(-1px, 2px) rotate(0.3deg);
    opacity: 0.8;
  }
}

.content {
  position: relative;
  z-index: 2;
  text-align: center;
  color: #FFFFFF;
  opacity: 0;
  transform: translateY(20px);
  transition: all 0.8s ease-in-out 0.3s;
}

.splash-container.loaded .content {
  opacity: 1;
  transform: translateY(0);
}

.logo-container {
  margin-bottom: 60px;
}

.logo-text {
  font-size: 8rem;
  font-weight: 900;
  font-style: italic;
  margin: 0;
  text-shadow: 
    0 0 10px rgba(0, 0, 0, 0.4),
    0 0 20px rgba(0, 0, 0, 0.2),
    0 0 30px rgba(0, 0, 0, 0.1),
    0 0 40px rgba(255, 255, 255, 0.3),
    0 0 60px rgba(184, 230, 255, 0.4);
  letter-spacing: 0.1em;
  filter: drop-shadow(0 0 15px rgba(255, 255, 255, 0.3));
}

.letter {
  display: inline-block;
  animation: letterGlow 3s ease-in-out infinite;
  transform-origin: center;
  will-change: transform, text-shadow;
}

@keyframes letterGlow {
  0%, 100% {
    transform: scale(1) rotate(0deg);
    text-shadow: 
      0 0 10px rgba(0, 0, 0, 0.4),
      0 0 20px rgba(0, 0, 0, 0.2),
      0 0 30px rgba(0, 0, 0, 0.1),
      0 0 40px rgba(255, 255, 255, 0.3),
      0 0 60px rgba(184, 230, 255, 0.4);
  }
  50% {
    transform: scale(1.03) rotate(0.5deg);
    text-shadow: 
      0 0 12px rgba(0, 0, 0, 0.5),
      0 0 25px rgba(0, 0, 0, 0.25),
      0 0 35px rgba(0, 0, 0, 0.12),
      0 0 50px rgba(255, 255, 255, 0.4),
      0 0 70px rgba(184, 230, 255, 0.6);
  }
}

.subtitle {
  font-size: 3rem;
  font-weight: 600;
  margin-top: 30px;
  opacity: 0.95;
  color: #FFFFFF;
  text-shadow: 
    0 0 8px rgba(0, 0, 0, 0.3),
    0 0 16px rgba(255, 255, 255, 0.2),
    0 0 24px rgba(184, 230, 255, 0.3);
  animation: subtitleFade 4s ease-in-out infinite;
  will-change: opacity, transform;
}

@keyframes subtitleFade {
  0%, 100% {
    opacity: 0.9;
    transform: translateY(0);
  }
  50% {
    opacity: 1;
    transform: translateY(-3px);
  }
}

@media (max-width: 768px) {
  .logo-text {
    font-size: 5rem;
  }
  
  .subtitle {
    font-size: 1.6rem;
  }
  
  .texture-layer {
    background-image: 
      linear-gradient(
        90deg,
        transparent 0%,
        transparent calc(50% - 0.8px),
        rgba(255, 255, 255, 0.07) calc(50% - 0.8px),
        rgba(255, 255, 255, 0.07) calc(50% + 0.8px),
        transparent calc(50% + 0.8px),
        transparent 100%
      ),
      linear-gradient(
        0deg,
        transparent 0%,
        transparent calc(50% - 0.8px),
        rgba(255, 255, 255, 0.07) calc(50% - 0.8px),
        rgba(255, 255, 255, 0.07) calc(50% + 0.8px),
        transparent calc(50% + 0.8px),
        transparent 100%
      ),
      repeating-linear-gradient(
        45deg,
        transparent 0%,
        transparent 96%,
        rgba(255, 255, 255, 0.035) 96%,
        rgba(255, 255, 255, 0.035) 100%
      );
  }
}

@media (max-width: 480px) {
  .logo-text {
    font-size: 4rem;
  }
  
  .subtitle {
    font-size: 1.3rem;
  }
  
  .texture-layer {
    background-image: 
      linear-gradient(
        90deg,
        transparent 0%,
        transparent calc(50% - 0.6px),
        rgba(255, 255, 255, 0.06) calc(50% - 0.6px),
        rgba(255, 255, 255, 0.06) calc(50% + 0.6px),
        transparent calc(50% + 0.6px),
        transparent 100%
      ),
      repeating-linear-gradient(
        45deg,
        transparent 0%,
        transparent 97%,
        rgba(255, 255, 255, 0.03) 97%,
        rgba(255, 255, 255, 0.03) 100%
      );
    animation: textureFloat 15s ease-in-out infinite;
  }
}

</style> 