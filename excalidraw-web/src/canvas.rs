use log::info;
use piet_web::WebRenderContext;
use wasm_bindgen::prelude::*;

pub fn create_context() -> WebRenderContext<'static> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let body = document.body().expect("document should have a body");

    let canvas = document
        .create_element("canvas")
        .expect("should create canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("should cast to canvas");
    let context = canvas
        .get_context("2d")
        .expect("should get context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("should cast to context");

    body.append_child(&canvas).expect("should append label");
    let dpr = window.device_pixel_ratio();
    info!("dpr: {}", dpr);
    let window_width = window.inner_width().unwrap().as_f64().unwrap();
    let window_height = window.inner_height().unwrap().as_f64().unwrap();
    canvas.set_width((window_width * dpr) as u32);
    canvas.set_height((window_height * dpr) as u32);
    canvas
        .style()
        .set_property("width", &format!("{}px", window_width))
        .expect("should set width");
    canvas
        .style()
        .set_property("height", &format!("{}px", window_height))
        .expect("should set height");

    let _ = context.scale(dpr, dpr);
    WebRenderContext::new(context, window)
}
