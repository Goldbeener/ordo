use fontdue::{Font, FontSettings};
use image::{Rgb, RgbImage};
use rusttype::{point, Font, Scale};
use std::fs::File;
use std::io::BufReader;

/// 生成指定颜色的图片，并在上面写入文本
pub fn generate_image_with_text(
    width: u32,
    height: u32,
    color: (u8, u8, u8),
    text: &str,
    output_path: &str,
    font_path: &str,
) -> Result<(), String> {
    // 创建指定颜色的图片
    // let mut img = RgbImage::from_pixel(width, height, Rgb([color.0, color.1, color.2]));

    // 读取字体
    // let absolute_font_path = std::fs::canonicalize(&font_path)
    //     .map_err(|_| format!("字体绝对路径转换失败: {:?}", font_path))?;
    // let font_data =
    //     std::fs::read(absolute_font_path).map_err(|e| format!("字体加载失败: {}", e))?;
    // let font = Font::try_from_vec(font_data).ok_or("字体解析失败")?;

    let mut img = RgbImage::new(width, height);

    // 设置背景色
    for pixel in img.pixels_mut() {
        *pixel = Rgb([30, 144, 255]); // 蓝色背景
    }

    let font_data = include_bytes!("../assets/fonts/Boldonse-Regular.ttf"); // 替换成你的字体文件
    let font = Font::from_bytes(font_data.as_ref(), FontSettings::default()).unwrap();

    // 设置字体大小
    let scale = Scale::uniform(50.0); // 文字大小

    // 计算文本起始位置
    let start_x = (width / 4) as i32;
    let start_y = (height / 2) as i32;

    // 绘制文本
    for glyph in font.layout(text, scale, point(start_x as f32, start_y as f32)) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let px = (x as i32 + bb.min.x) as u32;
                let py = (y as i32 + bb.min.y) as u32;
                if px < width && py < height {
                    let pixel = img.get_pixel_mut(px, py);
                    *pixel = Rgb([(255.0 * v) as u8, 255, 255]); // 白色文字
                }
            });
        }
    }

    // 保存图片
    img.save(output_path)
        .map_err(|e| format!("图片保存失败: {}", e))?;
    Ok(())
}
