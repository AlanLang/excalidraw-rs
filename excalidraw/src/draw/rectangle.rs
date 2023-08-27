use crate::{
    draw::options_generator::default_options_generator,
    element::{Element, Roundness, RoundnessType},
};
use log::debug;
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
    let path = match &element.roundness {
        Some(roundness) => {
            let w = element.width;
            let h = element.height;
            let r = get_corner_radius(w.min(h), roundness);
            let path = format!(
              "M {} 0 L {} 0 Q {} 0, {} {} L {} {} Q {} {}, {} {} L {} {} Q 0 {}, 0 {} L 0 {} Q 0 0, {} 0",
              r,
              w - r,
              w,
              w,r,w,h - r,w,h,w - r,h,r,h,h,h - r,r,r
          );
            generator.path::<f32>(path)
        }
        None => generator.rectangle::<f32>(0.0, 0.0, element.width, element.height),
    };
    let _ = ctx.save();
    ctx.transform(kurbo::Affine::translate((
        (element.x + config.offset_x) as f64,
        (element.y + config.offset_y) as f64,
    )));
    path.draw(ctx);
    let _ = ctx.restore();
}

fn get_corner_radius(x: f32, roundness: &Roundness) -> f32 {
    let default_proportional_radius = 0.25;
    match roundness.type_field {
        RoundnessType::Legacy => x * default_proportional_radius,
        RoundnessType::ProportionalRadius => x * default_proportional_radius,
        RoundnessType::AdaptiveRadius => {
            let fixed_radius_size = roundness.value.unwrap_or(32.0);
            let cutoff_size = fixed_radius_size / default_proportional_radius;
            if x <= cutoff_size {
                return x * default_proportional_radius;
            }

            return fixed_radius_size;
        }
    }
}
