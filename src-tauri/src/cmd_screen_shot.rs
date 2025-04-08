use base64::{engine::general_purpose, Engine as _};
use std::fs::{self, File};
use std::io::Write;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::{Duration, SystemTime};
use tauri::{AppHandle, Emitter, Manager};
use std::path::PathBuf;

// 全局存储截图处理任务通道
pub struct ScreenshotTaskManager(Sender<ScreenshotTask>);

struct ScreenshotTask {
    base64_data: String,
    save_path: String,
    format: String,
    app_handle: AppHandle,
    request_id: String, // 添加请求ID用于跟踪任务
}

// 初始化截图任务管理器
pub fn init_screenshot_manager() -> ScreenshotTaskManager {
    let (sender, receiver) = channel::<ScreenshotTask>();

    // 启动专用线程处理截图任务
    thread::spawn(move || {
        for task in receiver {
            // 处理每个截图任务
            let _ = process_screenshot_task(task);

            // 避免线程过度占用CPU
            thread::sleep(Duration::from_millis(50));
        }
    });

    ScreenshotTaskManager(sender)
}

// 删除上次壁纸
fn cleanup_old_wallpapers(dir: PathBuf, prefix: &str, max_age_minutes: u64) -> std::io::Result<()> {
    let entries = fs::read_dir(&dir)?;
    println!("Cleaning up old wallpapers at {}", dir.display());

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // 只清理我们自己创建的文件
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.starts_with(prefix) {
                let metadata = fs::metadata(&path)?;
                if let Ok(modified) = metadata.modified() {
                    let age = SystemTime::now()
                        .duration_since(modified)
                        .unwrap_or(Duration::ZERO);
                    if age > Duration::from_secs(max_age_minutes * 60) {
                        let _ = fs::remove_file(&path); // 忽略删除失败
                    }
                }
            }
        }
    }

    Ok(())
}

// 处理单个截图任务
fn process_screenshot_task(task: ScreenshotTask) -> Result<(), String> {
    let app_handle = task.app_handle.clone();
    let request_id = task.request_id.clone();

    // 获取应用数据目录
    let app_local_data = app_handle.path().app_local_data_dir().unwrap();

    // 构建完整的保存路径
    let full_path = app_local_data.join(&task.save_path);

    // 确保目录存在
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    println!("存储截图路径, {}!", full_path.display());
    // 保存之前先删除以前的临时文件
    // cleanup_old_wallpapers(full_path.clone(),"wallpaper-", 30).unwrap();

    // 将Base64数据解码为图像数据
    let image_data = general_purpose::STANDARD
        .decode(&task.base64_data)
        .map_err(|e| format!("Base64解码失败: {}", e))?;

    // 保存图像文件
    let mut file = File::create(&full_path).map_err(|e| format!("创建文件失败: {}", e))?;

    file.write_all(&image_data)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    println!("发送事件通知前端{}", request_id);
    // 发送事件通知前端保存完成
    let _ = app_handle.app_handle().emit(
        "screenshot-saved",
        ScreenshotSaveEvent {
            path: full_path.to_string_lossy().to_string(),
            format: task.format,
            success: true,
            request_id, // 包含请求ID在事件中
        },
    );
    println!("发送事件通知前端 ---完成");

    Ok(())
}

// 截图保存事件数据结构
#[derive(Clone, serde::Serialize)]
struct ScreenshotSaveEvent {
    path: String,
    format: String,
    success: bool,
    request_id: String, // 添加请求ID字段
}

// 处理截图保存命令
#[tauri::command]
pub async fn save_screenshot(
    app_handle: AppHandle,
    base64_data: String,
    save_path: String,
    format: String,
    request_id: String, // 添加请求ID参数
    task_manager: tauri::State<'_, ScreenshotTaskManager>,
) -> Result<(), String> {
    // 创建截图任务并发送到处理队列
    let task = ScreenshotTask {
        base64_data,
        save_path,
        format,
        app_handle: app_handle.clone(),
        request_id, // 设置请求ID
    };

    println!("接收到保存图片命令");
    // 发送任务到队列
    task_manager
        .0
        .send(task)
        .map_err(|e| format!("发送截图任务失败: {}", e))?;

    Ok(())
}
