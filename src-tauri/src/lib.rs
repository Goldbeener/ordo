// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod cmd_note_manage;
mod cmd_schedule;
mod cmd_screen_shot;
mod cmd_toggle_collapse;
mod cmd_toggle_layer;
mod cmd_wallpaper;
mod db;
mod db_timer;
mod generate_wallpaper;
mod init_app;

use cmd_note_manage::{
    create_note, delete_note, get_notes_count, list_notes, list_notes_by_date, list_notes_by_id,
    list_tagged_notes, update_note,
};
use cmd_schedule::{add_schedule, delete_schedule, get_schedules, init_timer_db};
use cmd_screen_shot::{init_screenshot_manager, save_screenshot};
use cmd_toggle_collapse::{collapse_window, expand_window};
use cmd_toggle_layer::toggle_always_on_top;
use cmd_wallpaper::gen_set_wallpaper;

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
            get_notes_count,
            create_note,
            delete_note,
            list_notes,
            list_tagged_notes,
            list_notes_by_date,
            list_notes_by_id,
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
