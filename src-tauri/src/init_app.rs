use std::sync::Mutex;
use tauri::{App, Manager, PhysicalPosition, PhysicalSize, Position};

use crate::db::NoteDatabase;

pub fn initialize_app(app: &mut App) -> Result<(), anyhow::Error> {
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;

    let db_path = app_data_dir.join("notes.db");
    let note_db = NoteDatabase::new(db_path.to_str().unwrap())?;

    app.manage(Mutex::new(note_db));
    let window = app.get_webview_window("main").unwrap();

    configure_window(window)?;

    Ok(())
}

fn configure_window(window: tauri::WebviewWindow) -> Result<(), anyhow::Error> {
    // 设置窗口层级
    window.set_always_on_top(true).unwrap();
    // 获取屏幕大小
    let primary_monitor = window.primary_monitor().unwrap().unwrap();
    let screen_size = primary_monitor.size();

    let fixed_width = 800.0; // 固定宽度
    let full_height = screen_size.height as f64 - 200.0; // 屏幕高度

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
}
