use crate::{draw::utils::default_options_generator, element::Element};
use piet::{kurbo, RenderContext};
use rough_piet::KurboGenerator;
use roughr::core::OptionsBuilder;

use super::DrawConfig;

pub fn draw(ctx: &mut impl RenderContext, element: &Element, config: &DrawConfig) {
    let mut options = OptionsBuilder::default();
    let options = default_options_generator(element, element.roundness.is_some(), &mut options)
        .curve_fitting(1.0)
        .build()
        .unwrap();
    let generator = KurboGenerator::new(options);
    let path = generator.ellipse::<f32>(
        element.width / 2 as f32,
        element.height / 2 as f32,
        element.width,
        element.height,
    );
    let _ = ctx.save();
    ctx.transform(kurbo::Affine::translate((
        (element.x + config.offset_x) as f64,
        (element.y + config.offset_y) as f64,
    )));
    path.draw(ctx);
    let _ = ctx.restore();
}
