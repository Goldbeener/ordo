use std::any::Any;
use std::sync::Mutex;
use tauri::{
    App, Manager, PhysicalPosition, PhysicalSize, Position, WindowEvent, Runtime, WebviewWindow,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent, TrayIcon},
};


use crate::db::NoteDatabase;

pub fn initialize_app(app: &mut App) -> Result<(), anyhow::Error> {
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;

    let db_path = app_data_dir.join("notes.db");
    let note_db = NoteDatabase::new(db_path.to_str().unwrap())?;

    app.manage(Mutex::new(note_db));
    let window = app.get_webview_window("main").unwrap();

    configure_window(window)?;
    setup_system_tray(&app.app_handle())?;

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

/// 创建托盘菜单
fn create_tray_menu<R: Runtime>(app: &tauri::AppHandle<R>) -> Result<Menu<R>, tauri::Error> {
    // 创建菜单项
    let quit_item = MenuItem::with_id(app, 'q', "退出", true, None::<&str>)?;
    let show_item = MenuItem::with_id(app, 's', "显示窗口", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, 'h', "隐藏窗口", true, None::<&str>)?;

    // 创建菜单并添加菜单项
    let menu = Menu::new(app)?;
    menu.append(&show_item)?;
    menu.append(&hide_item)?;
    menu.append(&quit_item)?;

    Ok(menu)
}

/// 配置菜单事件处理
fn handle_menu_events<R: Runtime>(app: &tauri::AppHandle<R>, event_id: &str) {
    match event_id {
        "q" => {
            println!("退出按钮被点击");
            app.exit(0);
        }
        "s" => {
            println!("显示窗口按钮被点击");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "h" => {
            println!("隐藏窗口按钮被点击");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }
        }
        _ => {
            println!("菜单项 {:?} 未处理", event_id);
        }
    }
}

/// 配置托盘图标事件处理
fn handle_tray_events<R: Runtime>(tray: &TrayIcon<R>, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            println!("托盘图标被左键点击");
            toggle_window_visibility(tray.app_handle().clone());
        }
        _ => {}
    }
}

/// 切换窗口可见性
fn toggle_window_visibility<R: Runtime>(app: tauri::AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

/// 设置窗口最小化到托盘
fn setup_window_to_tray<R: Runtime>(window: &WebviewWindow<R>, app_handle: tauri::AppHandle<R>) {
    let app_handle_clone = app_handle.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            // 阻止窗口真正关闭
            api.prevent_close();
            // 隐藏窗口
            if let Some(window) = app_handle_clone.get_webview_window("main") {
                let _ = window.hide();
            }
        }
    });
}

/// 创建系统托盘
fn setup_system_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> Result<(), tauri::Error> {
    // 创建托盘菜单
    let menu = create_tray_menu(app)?;

    // 配置和创建托盘
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("Ordo日记")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            handle_menu_events(app, (&event.id).as_ref());
        })
        .on_tray_icon_event(handle_tray_events)
        .build(app)?;

    // 获取主窗口并设置关闭时最小化到托盘
    if let Some(window) = app.get_webview_window("main") {
        setup_window_to_tray(&window, app.clone());
    }

    Ok(())
}
