use crate::{draw::utils::default_options_generator, element::Element};
use piet::{kurbo, RenderContext};
use rough_piet::KurboGenerator;
use roughr::core::OptionsBuilder;

use super::{utils::get_corner_radius, DrawConfig};

pub fn draw(ctx: &mut impl RenderContext, element: &Element, config: &DrawConfig) {
    let mut options = OptionsBuilder::default();
    let options = default_options_generator(element, element.roundness.is_some(), &mut options)
        .build()
        .unwrap();
    let generator = KurboGenerator::new(options);
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
