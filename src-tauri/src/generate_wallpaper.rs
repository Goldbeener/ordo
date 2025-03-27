use fontdue::{Font, FontSettings};
use image::{imageops::resize, Rgb, RgbImage};

/// 生成指定颜色的图片，并在上面写入文本
pub fn generate_image_with_text(width: u32, height: u32, output_path: &str) -> Result<(), String> {
    let scale_factor = 2;
    let mut img = RgbImage::new(width * scale_factor, height * scale_factor);

    // **使用内存填充背景色**
    img.pixels_mut().for_each(|p| *p = Rgb([30, 144, 255])); // **深蓝色背景**

    let font_data = include_bytes!("../assets/fonts/Boldonse-Regular.ttf"); // 替换成你的字体文件
    let font = Font::from_bytes(font_data.as_ref(), FontSettings::default()).unwrap();

    // 生成文字
    let text = "Hello, Tauri!";
    let font_size = 50.0;
    let mut x_cursor = 100; // 文字起始位置（可调整）
    let baseline = 300; // 基线位置

    // **逐个字符渲染**
    for c in text.chars() {
        let (metrics, bitmap) = font.rasterize(c, font_size); // 渲染单字符

        // **修正字符倒置问题**
        for y in 0..metrics.height {
            for x in 0..metrics.width {
                let pixel_index = y * metrics.width + x;
                let alpha = bitmap[pixel_index];

                if alpha > 0 {
                    let px = x_cursor + x as u32;
                    let py = baseline - metrics.height as u32 + y as u32;
                    if px < width && py < height {
                        let gray = alpha; // 直接使用 alpha 作为灰度值，模拟抗锯齿
                        img.put_pixel(px, py, Rgb([gray, gray, gray])); // 渐变灰色，柔化边缘
                    }
                }
            }
        }

        x_cursor += metrics.advance_width as u32; // 移动光标，准备绘制下一个字符
    }

    img = resize(&img, width, height, image::imageops::FilterType::Lanczos3);

    // 保存图片
    img.save(output_path)
        .map_err(|e| format!("图片保存失败: {}", e))?;
    Ok(())
}
