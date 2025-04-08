import { captureElementToDataUrl, captureElementToImage, setWallpaper } from '../utils/screenshotService.js'
export default  function useHandleCapture() {
    const targetElement = useTemplateRef('targetElement');
    const isCapturing = ref(false);
    const captureStatus = ref(null);
    // 将DOM元素保存为文件
    const captureToFile = async () => {
        if (isCapturing.value || !targetElement.value) return;

        try {
            isCapturing.value = true;
            captureStatus.value = { type: 'info', message: '正在处理截图...' };
            console.log('??? 正在处理截图...');

            const result = await captureElementToImage(targetElement.value.$el, {
                filePrefix: 'wallpaper',
                format: 'png',
                quality: 0.95,
                scale: 3, // 3倍分辨率，生成2K以上图片
                savePath: 'screenshots'
            });

            if (result.success) {
                console.log('???,截图已保存', result)
                await setWallpaper(result.path);
                captureStatus.value = { type: 'success', message: `截图已保存: ${result.fileName}` };
            } else {
                captureStatus.value = { type: 'error', message: `截图失败: ${result.error}` };
            }
        } catch (error) {
            console.error('截图过程出错:', error);
            captureStatus.value = { type: 'error', message: `截图错误: ${error.message}` };
        } finally {
            isCapturing.value = false;

            // 状态信息3秒后自动清除
            setTimeout(() => {
                if (captureStatus.value && captureStatus.value.type !== 'error') {
                    captureStatus.value = null;
                }
            }, 3000);
        }
    };

// 将DOM元素转换为数据URL
    const captureToDataUrl = async () => {
        if (isCapturing.value || !targetElement.value) return;

        try {
            isCapturing.value = true;
            captureStatus.value = { type: 'info', message: '正在处理截图...' };

            const result = await captureElementToDataUrl(targetElement.value.$el, {
                format: 'png',
                quality: 0.95,
                scale: 3 // 3倍分辨率
            });

            if (result.success) {
                captureStatus.value = { type: 'success', message: '截图数据已生成' };
            } else {
                captureStatus.value = { type: 'error', message: `截图失败: ${result.error}` };
            }
        } catch (error) {
            console.error('截图过程出错:', error);
            captureStatus.value = { type: 'error', message: `截图错误: ${error.message}` };
        } finally {
            isCapturing.value = false;

            // 状态信息3秒后自动清除
            setTimeout(() => {
                if (captureStatus.value && captureStatus.value.type !== 'error') {
                    captureStatus.value = null;
                }
            }, 3000);
        }
    };

    return {
        targetElement,
        captureToFile,
        captureToDataUrl
    }
}