<script setup>


import { onMounted, ref, onBeforeUnmount } from 'vue';
import router from '../router'
import WaveSurfer from 'wavesurfer.js'
import { exists, readDir, createDir, BaseDirectory, readBinaryFile, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import CryptoJS from 'crypto-js';
import { waveDecoder, waveEncoder } from '../utils/wave';
import { useUserStore } from '../stores/user';
import { invoke } from '@tauri-apps/api/tauri';
import NavBar from '../components/NavBar.vue';
import ReturnButton from '../components/ReturnButton.vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

// 语种配置
const languageMode = ref('zh_cn'); // 默认中文
const smallLangList = ref([
  { code: 'ar_il', name: t('language.ar_sa') },
  { code: 'es_es', name: t('language.es_es') },
  { code: 'fr_fr', name: t('language.fr_fr') },
  { code: 'ja_jp', name: t('language.ja_jp') },
  { code: 'de_DE', name: t('language.de_DE') }
]);
const selectedSmallLang = ref(''); // 初始为空，表示未选择
const showSmallLangSelect = ref(false);
const hasSelectedSmallLang = ref(false); // 跟踪是否已选择具体语种

const xfConfig = ref({
  hostUrl: "wss://iat-api.xfyun.cn/v2/iat",
  host: "iat-api.xfyun.cn",
  appid: "",
  apiSecret: "",
  apiKey: "",
  uri: "/v2/iat",
  highWaterMark: 1280
});

const enableASR = ref(true);

// 更新小语种列表的语言文本
const updateSmallLangList = () => {
  smallLangList.value = [
    { code: 'ar_il', name: t('language.ar_sa') },
    { code: 'es_es', name: t('language.es_es') },
    { code: 'fr_fr', name: t('language.fr_fr') },
    { code: 'ja_jp', name: t('language.ja_jp') },
    { code: 'de_DE', name: t('language.de_DE') }
  ];
};

// 切换语种
const toggleLanguage = async () => {
  if (languageMode.value === 'zh_cn') {
    languageMode.value = 'small_lang';
    xfConfig.value.hostUrl = "wss://iat-niche-api.xfyun.cn/v2/iat";
    xfConfig.value.host = "iat-niche-api.xfyun.cn";
    showSmallLangSelect.value = true; // 显示下拉菜单
    hasSelectedSmallLang.value = false; // 重置选择状态
  } else {
    languageMode.value = 'zh_cn';
    xfConfig.value.hostUrl = "wss://iat-api.xfyun.cn/v2/iat";
    xfConfig.value.host = "iat-api.xfyun.cn";
    showSmallLangSelect.value = false;
    hasSelectedSmallLang.value = false; // 重置选择状态
  }
  
  // 更新小语种列表的语言文本
  updateSmallLangList();
  
  // 只有切换到中英文时才重新加载当前音频的识别结果
  if (languageMode.value === 'zh_cn' && wavePathIndex.value !== -1) {
    const currentFilename = wavePathList.value[wavePathIndex.value];
    
    // 清空当前显示的文本
    waveTextOri.value = '';
    waveTextNew.value = '';
    
    // 重新加载当前音频的识别结果（会自动检查对应的文本文件）
    await revealSpeechRecognition(currentFilename, 0);
    await revealSpeechRecognition(currentFilename, 1);
  }
};

// 选择小语种
const selectSmallLang = async (code) => {
  selectedSmallLang.value = code;
  hasSelectedSmallLang.value = true; // 标记已选择具体语种
  showSmallLangSelect.value = false;
  
  // 选择小语种后重新加载当前音频的识别结果
  if (wavePathIndex.value !== -1) {
    const currentFilename = wavePathList.value[wavePathIndex.value];
    
    // 清空当前显示的文本
    waveTextOri.value = '';
    waveTextNew.value = '';
    
    // 重新加载当前音频的识别结果（会自动检查对应的文本文件）
    await revealSpeechRecognition(currentFilename, 0);
    await revealSpeechRecognition(currentFilename, 1);
  }
};

const loadConfig = async () => {
  try {
    const configContent = await readTextFile('config.json', { dir: BaseDirectory.AppConfig });
    const config = JSON.parse(configContent);
    xfConfig.value = { ...xfConfig.value, ...config.xfyun };
    enableASR.value = config.enableASR;
  } catch (error) {
    console.error('Error loading config:', error);
    router.push({ name: 'config' });
  }
};

const xfIAT = async (waveFilepath, resultFilepath, resultDisplay) => {
  if (!enableASR.value) {
    console.log('ASR 识别已禁用');
    return;
  }

  resultDisplay.value = ''
  
  const FRAME = {
    STATUS_FIRST_FRAME: 0,
    STATUS_CONTINUE_FRAME: 1,
    STATUS_LAST_FRAME: 2
  }

  let date = (new Date().toUTCString())
  let status = FRAME.STATUS_FIRST_FRAME
  let currentSid = ""
  let iatResult = []

  let wssUrl = xfConfig.value.hostUrl + "?authorization=" + getAuthStr(date) + "&date=" + date + "&host=" + xfConfig.value.host
  let ws = new WebSocket(wssUrl)

  ws.addEventListener('open', async (event) => {
    await initFileData()
  })

  ws.addEventListener('message', (event) => {
    let res = JSON.parse(event.data)
    if (res.code != 0) {
      console.error(`WebSocket错误: ${res.code}, ${res.message}`);
      return
    }

    let str = ""
    if (res.data.status == 2) {
      currentSid = res.sid
      ws.close()
    }

    iatResult[res.data.result.sn] = res.data.result
    if (res.data.result.pgs == 'rpl') {
      res.data.result.rg.forEach(i => {
        iatResult[i] = null
      })
    }
    iatResult.forEach(i => {
      if (i != null) {
        i.ws.forEach(j => {
          j.cw.forEach(k => {
            str += k.w
          })
        })
      }
    })
    resultDisplay.value = str
    if (res.data.status == 2) {
      writeTextFile(resultFilepath, resultDisplay.value, {
        dir: BaseDirectory.AppData
      });
    }
  })

  ws.addEventListener('close', () => {
    // WebSocket连接已关闭
  })

  ws.addEventListener('error', (err) => {
    console.error("WebSocket连接错误:", err)
  })

  async function initFileData() {
    try {
      const waveFile = await readBinaryFile(waveFilepath, {
        dir: BaseDirectory.AppData
      });
      const waveDecodeData = waveDecoder(waveFile);
      const { sampleRate, channelData } = waveDecodeData;

      let monoData;
      if (channelData.length > 1) {
        monoData = new Float32Array(channelData[0].length);
      for (let i = 0; i < channelData[0].length; i++) {
          let sum = 0;
          for (let ch = 0; ch < channelData.length; ch++) {
            sum += channelData[ch][i];
          }
          monoData[i] = sum / channelData.length;
        }
      } else {
        monoData = channelData[0];
      }

      let resampledData = monoData;
      if (sampleRate !== 16000) {
        const ratio = sampleRate / 16000;
        resampledData = new Float32Array(Math.floor(monoData.length / ratio));
        for (let i = 0; i < resampledData.length; i++) {
          resampledData[i] = monoData[Math.floor(i * ratio)];
        }
      }

      const pcmData = new Int16Array(resampledData.length);
      for (let i = 0; i < resampledData.length; i++) {
        pcmData[i] = Math.max(-32768, Math.min(32767, Math.floor(resampledData[i] * 32768)));
      }

      const frameSize = 1280;
      for (let offset = 0; offset < pcmData.length * 2; offset += frameSize) {
        const chunk = pcmData.slice(offset / 2, (offset + frameSize) / 2);
        
        if (offset + frameSize >= pcmData.length * 2) {
          status = FRAME.STATUS_LAST_FRAME;
        }
        
        const base64Data = arrayBufferToBase64(chunk.buffer);
        send(base64Data);
      }
    } catch (error) {
      console.error('Error processing audio file:', error);
    }
  }

  function arrayBufferToBase64(buffer) {
    const bytes = new Uint8Array(buffer);
    let binary = '';
    for (let i = 0; i < bytes.length; i++) {
      binary += String.fromCharCode(bytes[i]);
    }
    return btoa(binary);
  }

  function getAuthStr(date) {
    let signatureOrigin = `host: ${xfConfig.value.host}\ndate: ${date}\nGET ${xfConfig.value.uri} HTTP/1.1`
    let signatureSha = CryptoJS.HmacSHA256(signatureOrigin, xfConfig.value.apiSecret)
    let signature = CryptoJS.enc.Base64.stringify(signatureSha)
    let authorizationOrigin = `api_key="${xfConfig.value.apiKey}", algorithm="hmac-sha256", headers="host date request-line", signature="${signature}"`
    let authStr = CryptoJS.enc.Base64.stringify(CryptoJS.enc.Utf8.parse(authorizationOrigin))
    return authStr
  }

  function send(audioData) {
    let frame = "";
    let frameDataSection = {
      "status": status,
      "format": "audio/L16;rate=16000",
      "audio": audioData,
      "encoding": "raw"
    }
    
    if (status === FRAME.STATUS_FIRST_FRAME) {
        frame = {
          common: {
            app_id: xfConfig.value.appid
          },
          business: {
            language: languageMode.value === 'zh_cn' ? "zh_cn" : selectedSmallLang.value,
            domain: "iat",
            accent: "mandarin",
            ...(languageMode.value === 'zh_cn' && { dwa: "wpgs" }),
            ...(languageMode.value === 'zh_cn' && { vad_eos: 3000 })
          },
          data: frameDataSection
        }
        

        
        status = FRAME.STATUS_CONTINUE_FRAME;
    } else {
        frame = {
          data: frameDataSection
        }
    }
    
    ws.send(JSON.stringify(frame))
  }
}

const wavePathList = ref([])
const wavePathIndex = ref(-1)
const userStore = useUserStore()

const showContextMenu = ref(false)
const contextMenuX = ref(0)
const contextMenuY = ref(0)
const contextMenuIndex = ref(-1)

const showDeleteConfirm = ref(false)
const deleteIndex = ref(-1)

const showRenameDialog = ref(false)
const renameIndex = ref(-1)
const newFileName = ref('')

const handleContextMenu = (event, index) => {
  event.preventDefault()
  contextMenuX.value = event.clientX
  contextMenuY.value = event.clientY
  contextMenuIndex.value = index
  showContextMenu.value = true
}

const handleDelete = (index) => {
  showContextMenu.value = false
  deleteIndex.value = index
  showDeleteConfirm.value = true
}

const handleRename = (index) => {
  showContextMenu.value = false
  renameIndex.value = index
  newFileName.value = wavePathList.value[index]
  showRenameDialog.value = true
}

const confirmDelete = async () => {
  const filename = wavePathList.value[deleteIndex.value]
  try {
    const deleted = await invoke('delete_audio_file', {
      userId: userStore.user.id,
      filename: filename
    })
    
    if (deleted) {
      wavePathList.value.splice(deleteIndex.value, 1)
      if (wavePathList.value.length === 0) {
        wavePathIndex.value = -1
      } else if (deleteIndex.value === wavePathIndex.value) {
        wavePathIndex.value = Math.min(deleteIndex.value, wavePathList.value.length - 1)
        createWaveSurfer()
      }
    }
  } catch (error) {
    console.error('Error deleting audio file:', error)
  }
  showDeleteConfirm.value = false
}

const confirmRename = async () => {
  const oldFilename = wavePathList.value[renameIndex.value]
  try {
    const renamed = await invoke('rename_audio_file', {
      userId: userStore.user.id,
      oldFilename: oldFilename,
      newFilename: newFileName.value
    })
    
    if (renamed) {
      wavePathList.value[renameIndex.value] = newFileName.value
      if (renameIndex.value === wavePathIndex.value) {
        createWaveSurfer()
      }
    }
  } catch (error) {
    console.error('Error renaming audio file:', error)
  }
  showRenameDialog.value = false
}

const closeContextMenu = () => {
  showContextMenu.value = false;
};

const closeSmallLangSelect = (event) => {
  // 检查点击是否在语种切换按钮或下拉菜单内
  const languageToggle = event.target.closest('.language-toggle');
  const smallLangSelect = event.target.closest('.small-lang-select');
  
  if (!languageToggle && !smallLangSelect) {
    showSmallLangSelect.value = false;
  }
};

const init = async () => {
  if (!await exists('waves', { dir: BaseDirectory.AppData })) {
    await createDir('waves', { dir: BaseDirectory.AppData, recursive: true });
  } else {
    try {
      const audioFiles = await invoke('get_user_audio_files', { 
        userId: userStore.user.id 
      });
      wavePathList.value = audioFiles.map(file => file.filename);
    } catch (error) {
      console.error('Error loading audio files:', error);
    }
  }
  if (wavePathList.value.length !== 0) {
    wavePathIndex.value = 0;
  }
}

const wavesurferOri = ref(null)
const wavesurferNew = ref(null)

const createWaveSurfer = async () => {
  if (wavesurferOri.value) {
    wavesurferOri.value.destroy()
  }
  if (wavesurferNew.value) {
    wavesurferNew.value.destroy()
  }

  waveTextNew.value = ''
  waveTextOri.value = ''

  if (wavePathIndex.value === -1) {
    return
  }

  const filename = wavePathList.value[wavePathIndex.value];

  try {
    const hasAccess = await invoke('check_audio_file_access', {
      userId: userStore.user.id,
      filename: filename
    });

    if (!hasAccess) {
      console.error('No access to audio file');
      return;
    }
  } catch (error) {
    console.error('Error checking file access:', error);
    return;
  }

  if (!await exists('waves/' + filename + '_ori.wav', {
    dir: BaseDirectory.AppData
  })) {
    return
  }
  const waveFileOri = await readBinaryFile('waves/' + filename + '_ori.wav', {
    dir: BaseDirectory.AppData
  })

  revealSpeechRecognition(filename, 0);
  const waveFileBlobOri = new Blob([waveFileOri], { type: 'audio/wav' });
  const waveFileURLOri = window.URL.createObjectURL(waveFileBlobOri)

  wavesurferOri.value = WaveSurfer.create({
    container: wavesurferRefOri.value,
    waveColor: '#4294EC',
    progressColor: '#4294EC88',
    url: waveFileURLOri,
  })

  wavesurferOri.value.on('click', () => {
    wavesurferOri.value.play()
  })

  if (!await exists('waves/' + filename + '_new.wav', {
    dir: BaseDirectory.AppData
  })) {
    return
  }
  const waveFileNew = await readBinaryFile('waves/' + filename + '_new.wav', {
    dir: BaseDirectory.AppData
  })
  revealSpeechRecognition(filename, 1);
  const waveFileBlobNew = new Blob([waveFileNew], { type: 'audio/wav' });
  const waveFileURLNew = window.URL.createObjectURL(waveFileBlobNew)

  wavesurferNew.value = WaveSurfer.create({
    container: wavesurferRefNew.value,
    waveColor: '#4294EC',
    progressColor: '#4294EC88',
    url: waveFileURLNew,
  })

  wavesurferNew.value.on('click', () => {
    wavesurferNew.value.play()
  })
}

const changeWaveSurfer = async (index) => {
  // 如果之前有正在播放的音频，先暂停它们
  if (wavesurferOri.value) {
    wavesurferOri.value.pause()
  }
  if (wavesurferNew.value) {
    wavesurferNew.value.pause()
  }
  
  wavePathIndex.value = index;
  await createWaveSurfer();
  
  // 开始播放声音
  try {
    await invoke('start_audio_playback');
  } catch (error) {
    console.error('Error starting audio playback:', error);
  }
}

const wavesurferRefOri = ref()
const waveTextOri = ref('')
const wavesurferRefNew = ref()
const waveTextNew = ref('')

onMounted(async () => {
  await loadConfig();
  await init();
  await createWaveSurfer();
  
  // 初始化小语种列表的语言文本
  updateSmallLangList();
  
  // 添加音频播放结束的监听器
  if (wavesurferOri.value) {
    wavesurferOri.value.on('finish', async () => {
      try {
        await invoke('pause_audio_playback');
      } catch (error) {
        console.error('Error pausing audio playback:', error);
      }
    });
  }
  
  if (wavesurferNew.value) {
    wavesurferNew.value.on('finish', async () => {
      try {
        await invoke('pause_audio_playback');
      } catch (error) {
        console.error('Error pausing audio playback:', error);
      }
    });
  }
  
  document.addEventListener('click', closeContextMenu);
  document.addEventListener('click', closeSmallLangSelect);
  console.log('=== ListView 组件已加载 ===');
})

onBeforeUnmount(() => {
  // 组件卸载前暂停所有音频播放
  if (wavesurferOri.value) {
    wavesurferOri.value.pause()
  }
  if (wavesurferNew.value) {
    wavesurferNew.value.pause()
  }
  
  // 暂停音频输出
  invoke('pause_audio_playback').catch(error => {
    console.error('Error pausing audio playback:', error);
  });
  
  document.removeEventListener('click', closeContextMenu);
  document.removeEventListener('click', closeSmallLangSelect);
})

const revealSpeechRecognition = async (filename, type) => {
  // 如果是小语种模式但未选择具体语种，则不进行识别
  if (languageMode.value === 'small_lang' && !hasSelectedSmallLang.value) {
    console.log('=== 小语种模式未选择具体语种，跳过识别 ===');
    return;
  }

  let fullTextFilepath
  let fullWaveFilepath
  let waveTextRef

  // 根据当前语种模式确定文本文件路径
  const langSuffix = languageMode.value === 'zh_cn' ? '' : '_' + selectedSmallLang.value;
  
  if (type === 0) {
    fullTextFilepath = 'waves/' + filename + '_ori' + langSuffix + '.txt'
    fullWaveFilepath = 'waves/' + filename + '_ori.wav'
    waveTextRef = waveTextOri
  } else {
    fullTextFilepath = 'waves/' + filename + '_new' + langSuffix + '.txt'
    fullWaveFilepath = 'waves/' + filename + '_new.wav'  // 使用_new.wav文件进行转录
    waveTextRef = waveTextNew
  }

  // 检查对应的文本文件是否存在
  const shouldReRecognize = !await exists(fullTextFilepath, { dir: BaseDirectory.AppData });
  
  if (shouldReRecognize) {
    xfIAT(fullWaveFilepath, fullTextFilepath, waveTextRef)
  } else {
    const content = await readTextFile(fullTextFilepath, { dir: BaseDirectory.AppData })
    waveTextRef.value = content
  }
}

const btnReturn = () => {
  router.push({ name: 'home' })
}
</script>

<template>
  <div class="container">
    <NavBar />
    
    <div class="left fade-in-up">
      <div class="title fade-in-up">{{ $t('title') }}</div>
      <div v-if="wavePathIndex === -1" class="record-list fade-in-up">
      </div>
      <div v-else class="record-list fade-in-up">
        <div v-for="(item, index) in wavePathList" :key="index" class="record-item"
          :class="wavePathIndex === index ? 'active' : ''" 
          @click="changeWaveSurfer(index)"
          @contextmenu="handleContextMenu($event, index)">
          <div class="record-text">{{ item }}</div>
          <div v-if="wavePathIndex === index" class="record-border left_top"></div>
          <div v-if="wavePathIndex === index" class="record-border right_top"></div>
          <div v-if="wavePathIndex === index" class="record-border left_bottom"></div>
          <div v-if="wavePathIndex === index" class="record-border right_bottom"></div>
        </div>
      </div>
    </div>

    <div v-if="showContextMenu" class="context-menu" :style="{ 
      left: contextMenuX + 'px', 
      top: contextMenuY + 'px' 
    }">
      <div class="context-menu-item" @click="handleRename(contextMenuIndex)">
        {{ $t('list.rename') }}
      </div>
      <div class="context-menu-item" @click="handleDelete(contextMenuIndex)">
        {{ $t('list.delete') }}
      </div>
    </div>

    <div v-if="showDeleteConfirm" class="dialog-overlay">
      <div class="dialog">
        <h3>{{ $t('list.deleteConfirm') }}</h3>
        <p>{{ $t('list.deleteMessage') }}</p>
        <div class="dialog-buttons">
          <button class="btn-cancel" @click="showDeleteConfirm = false">
            {{ $t('list.cancel') }}
          </button>
          <button class="btn-delete" @click="confirmDelete">
            {{ $t('list.delete') }}
          </button>
        </div>
      </div>
    </div>

    <div v-if="showRenameDialog" class="dialog-overlay">
      <div class="dialog">
        <h3>{{ $t('list.renameConfirm') }}</h3>
        <p>{{ $t('list.renameMessage') }}</p>
        <div class="rename-input">
          <input type="text" v-model="newFileName" @keyup.enter="confirmRename">
        </div>
        <div class="dialog-buttons">
          <button class="btn-cancel" @click="showRenameDialog = false">
            {{ $t('list.cancel') }}
          </button>
          <button class="btn-save" @click="confirmRename">
            {{ $t('list.save') }}
          </button>
        </div>
      </div>
    </div>

    <div class="right fade-in-up">
      <div class="wave-ori fade-in-up">
        <div class="wave-title fade-in-up">{{ $t('list.waveTitleOri') }}</div>
        <div class="wave-graph fade-in-up" ref="wavesurferRefOri"></div>
        <div class="wave-text fade-in-up">
          <div>{{ waveTextOri }}</div>
        </div>
      </div>
      <div class="wave-new fade-in-up">
        <div class="wave-title fade-in-up">{{ $t('list.waveTitleNew') }}</div>
        <div class="wave-graph fade-in-up" ref="wavesurferRefNew"></div>
        <div class="wave-text fade-in-up">
          <div>{{ waveTextNew }}</div>
        </div>
      </div>
    </div>
    <ReturnButton class="fade-in-up" />
    <!-- 语种切换按钮 -->
    <div class="language-toggle fade-in-up" @click="toggleLanguage">
      <span class="language-text">{{ 
        languageMode === 'zh_cn' ? $t('language.zh_en') : 
        (hasSelectedSmallLang ? smallLangList.find(lang => lang.code === selectedSmallLang)?.name || $t('language.small_lang') : $t('language.small_lang'))
      }}</span>
    </div>
    <!-- 小语种下拉选择 -->
    <div v-if="showSmallLangSelect" class="small-lang-select">
      <div
        v-for="lang in smallLangList"
        :key="lang.code"
        class="small-lang-option"
        :class="{ active: selectedSmallLang === lang.code }"
        @click="selectSmallLang(lang.code)"
      >
        {{ lang.name }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  background-color: #FFFFFF;
  width: 100vw;
  height: 100vh;
  position: relative;
}

/* 返回按钮固定定位 */
:deep(.return) {
  position: fixed;
  left: 20px;
  bottom: 20px;
  z-index: 1000;
}

/* 语种切换按钮样式 */
.language-toggle {
  position: fixed;
  left: 140px; /* 在返回按钮右侧 */
  bottom: 20px; /* 与返回按钮同一水平线 */
  z-index: 1000;
  padding: 8px 20px;
  border-radius: 12px;
  background-color: #4294EC;
  color: #FFFFFF;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  user-select: none;
  min-width: 106px;
  justify-content: center;
}

.language-text {
  font-size: 16px;
  color: #FFFFFF;
  text-align: center;
}

.language-toggle:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(66, 148, 236, 0.4);
}

/* 小语种下拉选择样式 */
.small-lang-select {
  position: fixed;
  left: 140px;
  bottom: 80px;
  z-index: 1001;
  background: #fff;
  border: 1px solid #4294EC;
  border-radius: 8px;
  box-shadow: 0 2px 12px rgba(66, 148, 236, 0.15);
  min-width: 120px;
  padding: 6px 0;
}

.small-lang-option {
  padding: 8px 20px;
  cursor: pointer;
  color: #214D89;
  transition: all 0.2s ease;
}

.small-lang-option.active,
.small-lang-option:hover {
  background: #4294EC;
  color: #fff;
}

.left {
  width: 250px;
  padding: 15px 12px;
  padding-bottom: 100px;
  background: rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  display: flex;
  flex-direction: column;
  height: 100vh;
  box-sizing: border-box;
  margin-top: 80px;

  .title {
    margin: 10px 5px;
    color: #214D89;
    font-size: 24px;
    margin: 10px;
    padding-bottom: 10px;
    font-weight: bold;
    border-bottom: 1px solid #214D89;
  }

  .subtitle {
    color: #214D89;
    font-size: 16px;
    margin: 15px 5px;
    font-weight: bold;
  }

  .record-list {
    margin: 10px 5px;
    padding-right: 4px;
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    margin-bottom: 70px;

    &::-webkit-scrollbar {
      width: 6px;
    }

    &::-webkit-scrollbar-track {
      background: rgba(160, 210, 235, 0.2);
      border-radius: 3px;
    }

    &::-webkit-scrollbar-thumb {
      background: #A0D2EB;
      border-radius: 3px;

      &:hover {
        background: #88C0E0;
      }
    }

    .record-item {
      margin-right: 2px;
      padding: 8px 15px;
      line-height: 30px;
      border-radius: 12px;
      position: relative;
      background-color: #B0DAF3;
      transition: all 0.3s ease;

      &~.record-item {
        margin-top: 10px;
      }

      .record-text {
        color: #214D89;
        font-size: 16px;
      }

      &:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
      }

      &.active {
        background-color: #4294EC;        
        border: none;
        .record-text {
          color: #FFFFFF;
          font-weight: bold;
        }
      }
    }
  }
}

.right {
  flex: 1;
  margin: 10px;
  display: flex;
  flex-direction: column;
  gap: 30px;
  height: calc(100vh - 20px);
  box-sizing: border-box;
  margin-top: 100px;
}

.wave-title {
  height: 50px; 
  line-height: 50px;
  color: #214D89;
  font-size: 28px; 
  margin: 0 0 10px 0;  
  font-weight: bold;
  text-align: center;
  flex-shrink: 0;
}

.wave-graph {
  flex: 1;
  margin: 0;
  padding: 10px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  min-height: 80px;
  margin-bottom: 8px; 
}

.wave-text {
  height: 72px;      
  padding: 8px 12px;  
  border: 1px solid #4294EC;
  line-height: 1.2em;
  border-radius: 12px;
  position: relative;
  background: rgba(201, 228, 244, 0.6);
  color: #4294EC;
  font-size: 16px;
  overflow-y: auto;
  margin: 0;

  > div:first-child {
    max-height: 3.6em;
    word-break: break-word;
    white-space: pre-wrap;
  }

  &::-webkit-scrollbar {
    width: 6px;
  }

  &::-webkit-scrollbar-track {
    background: rgba(160, 210, 235, 0.2);
    border-radius: 3px;
    margin: 10px 0;
  }

  &::-webkit-scrollbar-thumb {
    background: #A0D2EB;
    border-radius: 3px;
    min-height: 24px;

    &:hover {
      background: #88C0E0;
    }
  }
}

.context-menu {
  position: fixed;
  background: white;
  border: 1px solid #ccc;
  border-radius: 12px;
  padding: 4px 0;
  width: 100px; 
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  z-index: 1000;
}

.context-menu-item {
  padding: 6px 12px; 
  cursor: pointer;
  color: #333;
  transition: background-color 0.2s;
  text-align: center;
  border-bottom: 1px solid #eee;
}

.context-menu-item:last-child {
  border-bottom: none;
}

.context-menu-item:hover {
  background-color: #f5f5f5;
}

.rename-input {
  margin: 20px 0;
  text-align: center;
}

.rename-input input {
  width: 80%;
  padding: 8px 12px;
  border: 1px solid #4294EC;
  border-radius: 6px;
  font-size: 16px;
  color: #214D89;
  background: white;
}

.rename-input input:focus {
  outline: none;
  border-color: #3a85d4;
  box-shadow: 0 0 0 2px rgba(66, 148, 236, 0.2);
}

.btn-save {
  padding: 8px 24px; 
  border: none;
  border-radius: 8px; 
  background: #4294EC;
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  font-weight: bold;
  min-width: 100px; 
}

.btn-save:hover {
  background: #3a85d4; 
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(66, 148, 236, 0.3);
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: #C9E4F4; 
  border-radius: 12px; 
  padding: 24px;
  min-width: 300px;
  max-width: 90%;
  border: 1px solid rgba(255, 255, 255, 0.3);
  box-shadow: 0 8px 32px rgba(33, 77, 137, 0.15);
}

.dialog h3 {
  margin: 0 0 16px;
  color: #214D89;
  font-weight: bold;
  text-align: center; 
  font-size: 20px; 
}

.dialog p {
  margin: 0 0 24px;
  color: #214D89;
  text-align: center; 
}

.dialog-buttons {
  display: flex;
  justify-content: center;  
  gap: 24px;  
  margin-top: 30px; 
}

.btn-cancel {
  padding: 8px 24px; 
  border: 1px solid #4294EC; 
  border-radius: 8px; 
  background: white;
  color: #4294EC; 
  cursor: pointer;
  transition: all 0.3s ease;
  font-weight: bold;
  min-width: 100px;
}

.btn-delete {
  padding: 8px 24px; 
  border: none;
  border-radius: 8px; 
  background: #4294EC;
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  font-weight: bold;
  min-width: 100px; 
}

.btn-cancel:hover {
  background: #f5f5f5;
}

.btn-delete:hover {
  background: #3a85d4; 
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(66, 148, 236, 0.3);
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

/* 左侧区域动画 - 从左侧进入 */
.left.fade-in-up {
  animation: slideIn-Left 0.8s ease-out 0.2s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 标题动画 */
.title.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.4s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 录音列表动画 */
.record-list.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.6s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 右侧区域动画 - 从右侧进入 */
.right.fade-in-up {
  animation: slideIn-Right 0.8s ease-out 0.3s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 波形容器动画 - 统一从下方进入 */
.wave-ori.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.5s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.wave-new.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.7s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 波形标题动画 */
.wave-title.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.6s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 波形图形动画 */
.wave-graph.fade-in-up {
  animation: scaleIn 0.8s ease-out 0.8s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 波形文本动画 */
.wave-text.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 0.9s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

/* 底部按钮动画 - 统一从下方进入 */
:deep(.return.fade-in-up) {
  animation: fadeIn-Up 0.6s ease-out 1.0s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}

.language-toggle.fade-in-up {
  animation: fadeIn-Up 0.6s ease-out 1.1s forwards;
  opacity: 0;
  animation-fill-mode: forwards;
}
</style>
