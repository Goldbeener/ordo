use tauri::Window;

pub fn toggle_always_on_top(window: Window, enable: bool) {
    window.set_always_on_top(enable).unwrap();
}
