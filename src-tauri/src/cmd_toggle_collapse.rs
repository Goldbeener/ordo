use tauri::{Manager, PhysicalPosition, PhysicalSize, Window};

// --- 常量定义 ---
// 初始展开宽度
const EXPANDED_WIDTH: f64 = 400.0;
// 折叠后的宽度
const COLLAPSED_WIDTH: f64 = 50.0;
// 动画过渡时间 (ms) - 主要给前端CSS使用, 后端调用是即时的
// const ANIMATION_DURATION: u32 = 300;

// 获取主显示器尺寸的辅助函数
fn get_primary_monitor_size(window: &Window) -> Option<PhysicalSize<u32>> {
    window.primary_monitor().ok().flatten().map(|m| m.size().clone())
}

// --- Tauri 命令 ---

#[tauri::command]
pub async fn collapse_window(window: Window) -> Result<(), String> {
    println!("Collapsing window...");
    if let Some(monitor_size) = get_primary_monitor_size(&window) {
        let scale_factor = window.scale_factor().unwrap_or(1.0);
        let collapsed_width_physical = (COLLAPSED_WIDTH * scale_factor).round() as u32;
        let height = window.outer_size().map_err(|e| e.to_string())?.height; // 保持当前高度

        let new_pos_x = monitor_size.width.saturating_sub(collapsed_width_physical);
        let current_pos_y = window.outer_position().map_err(|e| e.to_string())?.y; // 保持当前 Y 坐标

        // 1. 设置新位置 (确保紧贴右边缘)
        window.set_position(PhysicalPosition { x: new_pos_x as i32, y: current_pos_y })
            .map_err(|e| format!("Failed to set position: {}", e))?;
        println!("Set position to: ({}, {})", new_pos_x, current_pos_y);

        // 2. 设置新尺寸
        window.set_size(PhysicalSize { width: collapsed_width_physical, height })
            .map_err(|e| format!("Failed to set size: {}", e))?;
        println!("Set size to: ({}, {})", collapsed_width_physical, height);

        Ok(())
    } else {
        Err("Could not get primary monitor size".to_string())
    }
}

#[tauri::command]
pub async fn expand_window(window: Window) -> Result<(), String> {
    println!("Expanding window...");
    if let Some(monitor_size) = get_primary_monitor_size(&window) {
        let scale_factor = window.scale_factor().unwrap_or(1.0);
        let expanded_width_physical = (EXPANDED_WIDTH * scale_factor).round() as u32;
        let height = window.outer_size().map_err(|e| e.to_string())?.height; // 保持当前高度

        let new_pos_x = monitor_size.width.saturating_sub(expanded_width_physical);
        let current_pos_y = window.outer_position().map_err(|e| e.to_string())?.y; // 保持当前 Y 坐标


        // 1. 设置新位置 (确保展开后仍然贴近右边缘)
        // 注意：必须先设置位置，再设置大小，否则在某些系统上可能有问题
        window.set_position(PhysicalPosition { x: new_pos_x as i32, y: current_pos_y })
            .map_err(|e| format!("Failed to set position: {}", e))?;
        println!("Set position to: ({}, {})", new_pos_x, current_pos_y);


        // 2. 设置新尺寸
        window.set_size(PhysicalSize { width: expanded_width_physical, height })
            .map_err(|e| format!("Failed to set size: {}", e))?;
        println!("Set size to: ({}, {})", expanded_width_physical, height);

        Ok(())
    } else {
        Err("Could not get primary monitor size".to_string())
    }
}