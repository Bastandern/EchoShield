import axios from 'axios';

// 检查扰动库版本
export const checkPerturbationVersion = async () => {
  try {
    const versionUrl = import.meta.env.VITE_VERSION_CHECK_URL;
    if (!versionUrl) {
      console.warn('VITE_VERSION_CHECK_URL not configured');
      return null;
    }

    const response = await axios.get(versionUrl, {
      timeout: 5000,
    });

    if (response.status === 200) {
      return response.data;
    }
  } catch (error) {
    console.error('Failed to check perturbation version:', error);
  }
  return null;
};

// 获取最新版本的扰动文件URL
export const getLatestPerturbationUrl = async () => {
  const versionInfo = await checkPerturbationVersion();
  if (versionInfo && versionInfo.latest) {
    return `https://raw.githubusercontent.com/your-username/your-perturbation-library/main/assets/perturbations/${versionInfo.latest}/uap.ar`;
  }
  return null;
};

// 下载指定版本的扰动文件
export const downloadPerturbation = async (version = 'latest') => {
  try {
    const baseURL = import.meta.env.VITE_BASE_URL;
    const path = `/assets/perturbations/${version}/uap.ar`;
    
    const response = await axios.get(path, {
      baseURL: baseURL,
      timeout: 30000,
      responseType: 'arraybuffer',
    });

    if (response.status === 200) {
      return response.data;
    }
  } catch (error) {
    console.error('Failed to download perturbation file:', error);
    throw error;
  }
}; 