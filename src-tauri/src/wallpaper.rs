// src-tauri/src/wallpaper.rs
use std::fs;
use std::path::{Path, PathBuf};

pub fn set_wallpaper() -> Result<(), String> {
    // Construct the full path to the wallpaper image
    let wallpaper_path: PathBuf = Path::new("assets/wallpapers").join("starry-sky.jpg");

    // Validate the wallpaper file exists
    if !wallpaper_path.exists() {
        return Err(format!("壁纸文件不存在: {:?}", wallpaper_path));
    }

    // Convert path to absolute path
    let absolute_path = fs::canonicalize(&wallpaper_path)
        .map_err(|_| format!("绝对路径转换失败: {:?}", wallpaper_path))?;

    // Set wallpaper using wallpaper crate
    wallpaper::set_from_path(
        absolute_path
            .to_str()
            .ok_or_else(|| format!("壁纸设置失败: {:?}", wallpaper_path))?,
    )
    .map_err(|_e| format!("壁纸设置失败: {:?}", wallpaper_path))
}
