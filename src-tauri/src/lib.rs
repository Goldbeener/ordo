// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod cmd_note_manage;
mod cmd_toggle_layer;
mod db;
mod generate_wallpaper;
mod init_app;
mod wallpaper;

use cmd_note_manage::{create_note, delete_note, list_notes, update_note};
use cmd_toggle_layer::toggle_always_on_top;
use tauri::{App, Manager};

#[tauri::command]
fn set_wallpaper() -> Result<(), String> {
    // 图片宽高、颜色、文本、输出路径、字体路径
    let result =
        generate_wallpaper::generate_image_with_text(1920, 1080, "assets/wallpapers/current.png");

    if let Err(e) = result {
        eprintln!("生成图片失败: {}", e);
    } else {
        println!("图片生成成功: output.png");
    }
    wallpaper::set_wallpaper()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let _ = init_app::initialize_app(app);
            Ok(())
        })
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            toggle_always_on_top,
            create_note,
            delete_note,
            list_notes,
            update_note,
            set_wallpaper
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
