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
    let result = get_excalidraw(file_path)?;
    let buffer = draw_excalidraw(&result)?;
    let mut response = Response::new(Body::from(buffer));
    response
        .headers_mut()
        .insert("Content-Type", "image/png".parse()?);
    Ok(response)
}

fn get_excalidraw(file_path: &str) -> Result<Excalidraw> {
    let file = read_to_string(file_path)?;
    let result = Excalidraw::from_json(&file)?;
    Ok(result)
}

fn draw_excalidraw(excalidraw: &Excalidraw) -> Result<Vec<u8>> {
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

    let mut buffer = vec![0; width * height * 4];
    bitmap
        .copy_raw_pixels(ImageFormat::RgbaPremul, &mut buffer)
        .map_err(|e| anyhow::anyhow!("Piet error: {:?}", e))?;
    util::unpremultiply_rgba(&mut buffer);
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
    Ok(png_buffer)
}
