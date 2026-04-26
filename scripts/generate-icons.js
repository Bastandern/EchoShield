import sharp from 'sharp';
import path from 'path';
import { fileURLToPath } from 'url';
import fs from 'fs';
import { execSync } from 'child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const sourceIcon = path.join(__dirname, '../src-tauri/icons/icon.png');
const targetDir = path.join(__dirname, '../src-tauri/icons');

// 定义需要生成的图标尺寸
const sizes = {
  '32x32.png': 32,
  '128x128.png': 128,
  '128x128@2x.png': 256,
  'Square30x30Logo.png': 30,
  'Square44x44Logo.png': 44,
  'Square71x71Logo.png': 71,
  'Square89x89Logo.png': 89,
  'Square107x107Logo.png': 107,
  'Square142x142Logo.png': 142,
  'Square150x150Logo.png': 150,
  'Square284x284Logo.png': 284,
  'Square310x310Logo.png': 310,
  'StoreLogo.png': 50
};

// macOS图标尺寸
const macSizes = [16, 32, 64, 128, 256, 512, 1024];

// Windows ICO尺寸
const icoSizes = [16, 24, 32, 48, 64, 128, 256];

async function generateIcons() {
  try {
    // 确保源图标存在
    if (!fs.existsSync(sourceIcon)) {
      console.error('Source icon not found:', sourceIcon);
      return;
    }

    // 读取源图标
    const image = sharp(sourceIcon);

    // 生成不同尺寸的图标
    for (const [filename, size] of Object.entries(sizes)) {
      const targetPath = path.join(targetDir, filename);
      console.log(`Generating ${filename} (${size}x${size})...`);
      
      await image
        .clone()
        .resize(size, size, {
          fit: 'contain',
          background: { r: 0, g: 0, b: 0, alpha: 0 }
        })
        .toFile(targetPath);
    }

    // 生成macOS图标
    const iconsetDir = path.join(targetDir, 'icon.iconset');
    if (!fs.existsSync(iconsetDir)) {
      fs.mkdirSync(iconsetDir);
    }

    for (const size of macSizes) {
      const filename = `icon_${size}x${size}.png`;
      const filename2x = `icon_${size/2}x${size/2}@2x.png`;
      const targetPath = path.join(iconsetDir, size >= 512 ? filename : filename2x);
      
      console.log(`Generating ${filename} (${size}x${size})...`);
      
      await image
        .clone()
        .resize(size, size, {
          fit: 'contain',
          background: { r: 0, g: 0, b: 0, alpha: 0 }
        })
        .toFile(targetPath);
    }

    // 使用iconutil将iconset转换为icns（仅在macOS上可用）
    try {
      execSync(`iconutil -c icns "${iconsetDir}" -o "${path.join(targetDir, 'icon.icns')}"`);
      console.log('Generated icon.icns');
    } catch (error) {
      console.log('Could not generate icon.icns (requires macOS)');
    }

    // 清理临时文件
    if (fs.existsSync(iconsetDir)) {
      fs.rmSync(iconsetDir, { recursive: true });
    }

    // 生成Windows ICO文件
    console.log('Generating Windows ICO file...');
    const icoBuffers = await Promise.all(
      icoSizes.map(size => 
        image
          .clone()
          .resize(size, size, {
            fit: 'contain',
            background: { r: 0, g: 0, b: 0, alpha: 0 }
          })
          .toFormat('png')
          .toBuffer()
      )
    );

    // 创建ICO文件头
    const headerSize = 6 + icoSizes.length * 16;
    const header = Buffer.alloc(headerSize);
    
    // 写入ICO头部
    header.writeUInt16LE(0, 0); // Reserved
    header.writeUInt16LE(1, 2); // ICO type
    header.writeUInt16LE(icoSizes.length, 4); // Number of images

    // 计算每个图像的偏移量
    let offset = headerSize;
    let imageHeaders = [];

    // 写入每个图像的头部信息
    icoBuffers.forEach((buffer, index) => {
      const size = icoSizes[index];
      const imageHeader = Buffer.alloc(16);
      // 对于大于255的尺寸，写入0表示256
      imageHeader.writeUInt8(size > 255 ? 0 : size, 0); // Width
      imageHeader.writeUInt8(size > 255 ? 0 : size, 1); // Height
      imageHeader.writeUInt8(0, 2); // Color palette
      imageHeader.writeUInt8(0, 3); // Reserved
      imageHeader.writeUInt16LE(1, 4); // Color planes
      imageHeader.writeUInt16LE(32, 6); // Bits per pixel
      imageHeader.writeUInt32LE(buffer.length, 8); // Image size
      imageHeader.writeUInt32LE(offset, 12); // Image offset
      imageHeaders.push(imageHeader);
      offset += buffer.length;
    });

    // 将所有部分组合成最终的ICO文件
    const ico = Buffer.concat([
      header,
      ...imageHeaders,
      ...icoBuffers
    ]);

    // 写入ICO文件
    fs.writeFileSync(path.join(targetDir, 'icon.ico'), ico);
    console.log('Generated icon.ico');

    console.log('All icons generated successfully!');
  } catch (error) {
    console.error('Error generating icons:', error);
  }
}

generateIcons(); 