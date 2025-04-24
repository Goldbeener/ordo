// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod cmd_note_manage;
mod cmd_screen_shot;
mod cmd_toggle_layer;
mod cmd_wallpaper;
mod db;
mod generate_wallpaper;
mod init_app;

mod db_timer;

mod cmd_schedule;
mod cmd_toggle_collapse;

use cmd_note_manage::{create_note, delete_note, list_notes, update_note};
use cmd_screen_shot::{init_screenshot_manager, save_screenshot};
use cmd_toggle_layer::toggle_always_on_top;
use cmd_wallpaper::gen_set_wallpaper;
use cmd_toggle_collapse::{ expand_window, collapse_window };
use cmd_schedule::{add_schedule, get_schedules, delete_schedule, init_timer_db};

use std::fs;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化截图任务管理器
    let screenshot_manager = init_screenshot_manager();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(screenshot_manager)
        .setup(|app| {

            let _ = init_app::initialize_app(app);

            // 初始化定时任务
            init_timer_db(app)?;

            // 确保应用数据目录中的screenshots文件夹存在
            let app_data_dir = app
                .path()
                .app_local_data_dir()
                .expect("Failed to get app data directory");

            let screenshots_dir = app_data_dir.join("screenshots");
            fs::create_dir_all(screenshots_dir).expect("Failed to create screenshots directory");

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            toggle_always_on_top,
            expand_window,
            collapse_window,
            create_note,
            delete_note,
            list_notes,
            update_note,
            save_screenshot,
            gen_set_wallpaper,
            add_schedule,
            get_schedules,
            delete_schedule,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
