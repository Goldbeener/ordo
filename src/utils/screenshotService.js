// src/services/screenshotService.js
import html2canvas from 'html2canvas';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { appLocalDataDir } from '@tauri-apps/api/path';
import { listen } from '@tauri-apps/api/event';

// 监听截图保存完成事件
let listenerInitialized = false;
const saveCompletedCallbacks = new Map();

const initializeListener = () => {
    if (listenerInitialized) return;

    listen('screenshot-saved', (event) => {

        console.log('收到后端的信息>>>',event.payload)
        const { path, format, success, request_id: requestId } = event.payload;

        // 查找对应的回调函数
        const callback = saveCompletedCallbacks.get(requestId);
        if (callback) {
            callback({ path, format, success });
            saveCompletedCallbacks.delete(requestId); // 清理回调
        }
    });

    listenerInitialized = true;
};

/**
 * 生成唯一请求ID
 */
const generateRequestId = () => {
    return `req_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;
};

/**
 * 生成文件名
 * @param {string} prefix - 文件名前缀
 * @param {string} format - 文件格式
 * @returns {string} 生成的文件名
 */
const generateFileName = (prefix = 'screenshot', format = 'png') => {
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    return `${prefix}-${timestamp}.${format}`;
};

/**
 * 将DOM元素转换为高质量图片并保存
 * @param {HTMLElement} element - 需要截图的DOM元素
 * @param {Object} options - 截图选项
 * @param {string} [options.filePrefix='screenshot'] - 文件名前缀
 * @param {string} [options.format='png'] - 图片格式，'png'或'jpeg'
 * @param {number} [options.quality=0.95] - 图片质量 (0-1, 仅对JPEG有效)
 * @param {number} [options.scale=2] - 图片缩放比例，用于提高分辨率
 * @param {string} [options.savePath='screenshots'] - 保存路径 (相对于应用数据目录)
 * @returns {Promise<Object>} 包含截图结果信息的Promise
 */
export const captureElementToImage = async (element, options = {}) => {
    if (!element || !(element instanceof HTMLElement)) {
        throw new Error('无效的DOM元素');
    }

    // 初始化监听器（如果尚未初始化）
    initializeListener();

    // 合并默认选项
    const defaultOptions = {
        filePrefix: 'screenshot',
        format: 'png',
        quality: 0.95,
        scale: 2,
        savePath: 'screenshots'
    };

    const settings = { ...defaultOptions, ...options };

    try {
        // 使用html2canvas捕获DOM元素
        const canvas = await html2canvas(element, {
            width: 1920,
            height: 1080,
            x: -200,
            y: -200,
            backgroundColor: "#ffffff",
            scale: settings.scale,
            useCORS: true,
            logging: false,
            allowTaint: true,
        });

        // 转换为base64数据
        const dataUrl = canvas.toDataURL(`image/${settings.format}`, settings.quality);
        const base64Data = dataUrl.split(',')[1];

        // 获取应用数据目录
        const appDataDir = await appLocalDataDir();
        const fileName = generateFileName(settings.filePrefix, settings.format);

        // 生成请求ID用于追踪保存结果
        const requestId = generateRequestId();


        // 创建一个Promise用于等待保存完成
        const savePromise = new Promise((resolve) => {
            saveCompletedCallbacks.set(requestId, (result) => {
                if (result.success) {
                    resolve({
                        success: true,
                        fileName,
                        path: result.path,
                        url: convertFileSrc(result.path),
                        width: canvas.width,
                        height: canvas.height
                    });
                } else {
                    resolve({
                        success: false,
                        error: '保存失败'
                    });
                }
            });

            // 设置超时处理
            setTimeout(() => {
                if (saveCompletedCallbacks.has(requestId)) {
                    saveCompletedCallbacks.delete(requestId);
                    resolve({
                        success: false,
                        error: '保存超时'
                    });
                }
            }, 30000); // 30秒超时
        });

        // 调用后端保存图片
        await invoke('save_screenshot', {
            base64Data,
            savePath: `${settings.savePath}/${fileName}`,
            format: settings.format,
            requestId
        });

        // 等待保存结果
        return await savePromise;

    } catch (error) {
        console.error('截图失败:', error);
        return {
            success: false,
            error: error.message
        };
    }
};

/**
 * 将DOM元素转换为高质量图片并获取数据URL (不保存为文件)
 * @param {HTMLElement} element - 需要截图的DOM元素
 * @param {Object} options - 截图选项
 * @param {string} [options.format='png'] - 图片格式，'png'或'jpeg'
 * @param {number} [options.quality=0.95] - 图片质量 (0-1, 仅对JPEG有效)
 * @param {number} [options.scale=2] - 图片缩放比例，用于提高分辨率
 * @returns {Promise<Object>} 包含数据URL的Promise
 */
export const captureElementToDataUrl = async (element, options = {}) => {
    if (!element || !(element instanceof HTMLElement)) {
        throw new Error('无效的DOM元素');
    }

    // 合并默认选项
    const defaultOptions = {
        format: 'png',
        quality: 0.95,
        scale: 2
    };

    const settings = { ...defaultOptions, ...options };

    try {
        // 使用html2canvas捕获DOM元素
        const canvas = await html2canvas(element, {
            scale: settings.scale,
            useCORS: true,
            logging: false,
            backgroundColor: null,
            allowTaint: true,
        });

        // 转换为dataURL
        const dataUrl = canvas.toDataURL(`image/${settings.format}`, settings.quality);

        return {
            success: true,
            dataUrl,
            width: canvas.width,
            height: canvas.height
        };
    } catch (error) {
        console.error('截图失败:', error);
        return {
            success: false,
            error: error.message
        };
    }
};

export const setWallpaper = async (path) => {
    await invoke("gen_set_wallpaper", { path });
}