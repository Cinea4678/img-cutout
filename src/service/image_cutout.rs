use std::{collections::VecDeque, io::Cursor};

use axum::body::Bytes;
use image::{DynamicImage, GenericImageView};

fn is_similar_to_white(pixel: &[u8], tolerance: u8) -> bool {
    pixel[0] >= 255 - tolerance &&
    pixel[1] >= 255 - tolerance &&
    pixel[2] >= 255 - tolerance
}

// 白色背景 静态图
pub fn image_cutout_static_white(image: &Bytes, tolerance: u8) -> Result<Vec<u8>, image::ImageError>  {
    let img = image::load_from_memory(&image).unwrap();
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8(); // 转成 RGBA 格式，方便后续操作

    // 2. 标记已访问像素
    let mut visited = vec![vec![false; height as usize]; width as usize];

    // 3. 初始化边缘白色像素入队
    let mut stack = VecDeque::new();

    // 遍历四条边
    for x in 0..width {
        for &y in [0, height - 1].iter() {
            let pixel = img.get_pixel(x, y).0;
            if is_similar_to_white(&pixel, tolerance) {
                stack.push_back((x, y));
                visited[x as usize][y as usize] = true;
            }
        }
    }
    for y in 0..height {
        for &x in [0, width - 1].iter() {
            let pixel = img.get_pixel(x, y).0;
            if is_similar_to_white(&pixel, tolerance) {
                stack.push_back((x, y));
                visited[x as usize][y as usize] = true;
            }
        }
    }

    // 4. 使用 DFS（其实这里是栈的 BFS，原理类似）处理白色区域
    while let Some((x, y)) = stack.pop_back() {
        // 将当前像素设置为透明
        img.put_pixel(x, y, image::Rgba([0, 0, 0, 0]));

        // 方向: 上、下、左、右
        let directions = [
            (0i32, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
        ];

        for (dx, dy) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                let (nx, ny) = (nx as u32, ny as u32);
                if !visited[nx as usize][ny as usize] {
                    let neighbor_pixel = img.get_pixel(nx, ny).0;
                    if is_similar_to_white(&neighbor_pixel, tolerance) {
                        stack.push_back((nx, ny));
                        visited[nx as usize][ny as usize] = true;
                    }
                }
            }
        }
    }

    // 5. 最后将处理后的图片保存到内存中
    let mut output = Vec::new();
    let mut output_cursor = Cursor::new(&mut output);
    DynamicImage::ImageRgba8(img).write_to(&mut output_cursor, image::ImageFormat::Png)?;

    Ok(output)
}