mod canvas;
extern crate cfg_if;
extern crate wasm_bindgen;
use cfg_if::cfg_if;
use excalidraw::Excalidraw;
use log::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen_console_logger::DEFAULT_LOGGER;
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    } else if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use std::panic;
        #[wasm_bindgen]
        pub fn set_panic_hook() {
            panic::set_hook(Box::new(console_error_panic_hook::hook));
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    log::set_logger(&DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let mut piet_context = canvas::create_context();

    Excalidraw::default().draw(&mut piet_context, 0 as f32);

    info!("Informational message");
}
