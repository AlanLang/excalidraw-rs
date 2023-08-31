extern crate dotenv;
use anyhow::Result;
use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use excalidraw::Excalidraw;
use log::debug;
use piet_common::{kurbo::Rect, util, Color, Device, ImageFormat, RenderContext, StrokeStyle};
use png::{ColorType, Encoder};
use std::{fs::read_to_string, io::Cursor, net::SocketAddr};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    debug!("Starting up");

    let app = Router::new()
        .route("/", get(root))
        .route("/file/*path", get(image_file));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3300));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn image_file(Path(path): Path<String>) -> impl IntoResponse {
    let file_path = format!("files/{}", path); // 请确保你有一个名为 `relative_directory` 的目录，并且里面有你想要访问的文件。
    debug!("file_path: {}", file_path);

    match make_electrical_diagram(&file_path) {
        Ok(content) => content,
        Err(e) => {
            let not_found = e.to_string();
            let mut res = Response::new(Body::from(not_found));
            *res.status_mut() = StatusCode::NOT_FOUND;
            res
        }
    }
}

fn make_electrical_diagram(file_path: &str) -> Result<Response<Body>> {
    let file = read_to_string(file_path)?;
    let hash1 = blake3::hash(&file.as_bytes());

    let hash_file_name = format!("{}.txt", file_path);
    let image_file_name = format!("{}.png", file_path);

    let saved_hash = match read_to_string(&hash_file_name) {
        Ok(content) => content,
        Err(_) => String::new(),
    };

    if saved_hash == hash1.to_hex().to_string() {
        // 如果系统有这个文件就直接返回
        if std::path::Path::new(&image_file_name).exists() {
            let buffer = std::fs::read(&image_file_name)?;
            let mut response = Response::new(Body::from(buffer));
            response
                .headers_mut()
                .insert("Content-Type", "image/png".parse()?);
            return Ok(response);
        }
    }

    let result = Excalidraw::from_json(&file)?;

    let buffer = draw_excalidraw(&result, &image_file_name)?;
    let mut response = Response::new(Body::from(buffer));
    response
        .headers_mut()
        .insert("Content-Type", "image/png".parse()?);
    // 保存文件
    std::fs::write(&hash_file_name, hash1.to_hex().to_string())?;
    Ok(response)
}

fn draw_excalidraw(excalidraw: &Excalidraw, file_name: &str) -> Result<Vec<u8>> {
    debug!("开始绘制");
    let padding = 100 as f32;
    let mut device = Device::new().map_err(|e| anyhow::anyhow!("Piet error: {:?}", e))?;
    let rect = excalidraw.get_canvas_size();
    println!("rect: {:?}", rect);
    let scale_factor = 4.0; // 4倍的缩放因子
    let width = ((rect.width + padding * 2 as f32) as f64 * scale_factor) as usize;
    let height = ((rect.height + padding * 2 as f32) as f64 * scale_factor) as usize;
    debug!("width: {}, height: {}", width, height);
    let mut bitmap = device
        .bitmap_target(width, height, scale_factor)
        .map_err(|e| anyhow::anyhow!("Piet error: {:?}", e))?;
    let mut rc = bitmap.render_context();
    let background_color = Color::from_hex_str("FFFFFF").unwrap();
    rc.fill(
        Rect::new(0.0, 0.0, width as f64, height as f64),
        &background_color,
    );
    let stroke_style = StrokeStyle::new()
        .line_join(piet_common::LineJoin::Round)
        .line_cap(piet_common::LineCap::Round);
    rc.stroke_styled(
        Rect::new(0.0, 0.0, width as f64, height as f64),
        &background_color,
        width as f64,
        &stroke_style,
    );
    // 这里可能需要手动将绘图指令也放大两倍
    excalidraw.draw(&mut rc, padding);
    rc.finish()
        .map_err(|e| anyhow::anyhow!("Piet error: {:?}", e))?;
    std::mem::drop(rc);
    debug!("绘制完成");

    let mut buffer = vec![0; width * height * 4];
    bitmap
        .copy_raw_pixels(ImageFormat::RgbaPremul, &mut buffer)
        .map_err(|e| anyhow::anyhow!("Piet error: {:?}", e))?;
    util::unpremultiply_rgba(&mut buffer);
    bitmap.save_to_file(file_name).expect("file save error");
    let mut png_buffer = Vec::new();
    {
        let writer = Cursor::new(&mut png_buffer);
        let mut encoder = Encoder::new(writer, width as u32, height as u32);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder
            .write_header()
            .map_err(|e| anyhow::anyhow!("Error writing PNG header: {:?}", e))?;
        writer
            .write_image_data(&buffer)
            .map_err(|e| anyhow::anyhow!("Error writing PNG image data: {:?}", e))?;
    }
    debug!("生成图片");
    Ok(png_buffer)
}
