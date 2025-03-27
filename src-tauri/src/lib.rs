// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Manager, PhysicalPosition, PhysicalSize, Position, Window};

mod generate_wallpaper;
mod toggle_layer;
mod wallpaper;

#[tauri::command]
fn toggle_always_on_top(window: Window, enable: bool) {
    println!("设置成功{}", enable);
    toggle_layer::toggle_always_on_top(window, enable);
}

#[tauri::command]
fn set_wallpaper() -> Result<(), String> {
    // 图片宽高、颜色、文本、输出路径、字体路径
    // let result =
    //     generate_wallpaper::generate_image_with_text(1920, 1080, "assets/wallpapers/current.png");

    // if let Err(e) = result {
    //     eprintln!("生成图片失败: {}", e);
    // } else {
    //     println!("图片生成成功: output.png");
    // }
    wallpaper::set_wallpaper()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // 屏幕始终在最高层级
            window.set_always_on_top(true).unwrap();

            // 获取屏幕大小
            let primary_monitor = window.primary_monitor().unwrap().unwrap();
            let screen_size = primary_monitor.size();

            let fixed_width = 800.0; // 固定宽度
            let full_height = screen_size.height as f64; // 屏幕高度

            // 设置窗口大小
            window
                .set_size(PhysicalSize::new(fixed_width, full_height))
                .unwrap();

            // 可选：让窗口靠右
            let x = (screen_size.width as f64 - fixed_width) as i32;
            let y = 80;
            window
                .set_position(Position::Physical(PhysicalPosition::new(x, y)))
                .unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            toggle_always_on_top,
            set_wallpaper
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
