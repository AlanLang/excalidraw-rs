use crate::{
    draw::options_generator::default_options_generator,
    element::{Element, StrokeStyle},
};
use log::debug;
use palette::Srgba;
use piet::{kurbo, RenderContext};
use rough_piet::KurboGenerator;
use roughr::core::OptionsBuilder;

use super::DrawConfig;

pub fn draw(ctx: &mut impl RenderContext, element: &Element, config: &DrawConfig) {
    let mut options = OptionsBuilder::default();
    let options = default_options_generator(element, element.roundness.is_some(), &mut options)
        .build()
        .unwrap();
    let generator = KurboGenerator::new(options);
    debug!("element: {:?}", element);
    debug!("config: {:?}", config);
    let (top_x, top_y, right_x, right_y, bottom_x, bottom_y, left_x, left_y) =
        get_diamond_points(element);
    todo!("diamond");
    let path = match &element.roundness {
        Some(roundness) => {
            let w = element.width;
            let h = element.height;
            let r = 0.0;
            let path = format!(
              "M {} 0 L {} 0 Q {} 0, {} {} L {} {} Q {} {}, {} {} L {} {} Q 0 {}, 0 {} L 0 {} Q 0 0, {} 0",
              r,
              w - r,
              w,
              w,r,w,h - r,w,h,w - r,h,r,h,h,h - r,r,r
          );
            generator.path::<f32>(path)
        }
        None => generator.polygon::<f32>(0.0, 0.0, element.width, element.height),
    };
    let _ = ctx.save();
    ctx.transform(kurbo::Affine::translate((
        (element.x + config.offset_x) as f64,
        (element.y + config.offset_y) as f64,
    )));
    path.draw(ctx);
    let _ = ctx.restore();
}

fn get_diamond_points(element: &Element) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
    let top_x = (element.width / 2 as f32).floor() + 1 as f32;
    let top_y: f32 = 0.0;
    let right_x = element.width;
    let right_y = (element.height / 2 as f32).floor() + 1 as f32;
    let bottom_x = top_x;
    let bottom_y = element.height;
    let left_x = 0.0;
    let left_y = right_y;

    (
        top_x, top_y, right_x, right_y, bottom_x, bottom_y, left_x, left_y,
    )
}
