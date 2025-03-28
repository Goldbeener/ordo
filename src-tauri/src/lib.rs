// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod cmd_note_manage;
mod cmd_toggle_layer;
mod cmd_wallpaper;
mod db;
mod generate_wallpaper;
mod init_app;

use cmd_note_manage::{create_note, delete_note, list_notes, update_note};
use cmd_toggle_layer::toggle_always_on_top;
use cmd_wallpaper::gen_set_wallpaper;
use tauri::{App, Manager};

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
            gen_set_wallpaper
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
