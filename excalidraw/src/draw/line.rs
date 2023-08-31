use crate::{draw::utils::default_options_generator, element::Element, point::Point};
use palette::Srgba;
use piet::{kurbo, RenderContext};
use rough_piet::KurboGenerator;
use roughr::core::OptionsBuilder;

use super::{utils::get_points2d, DrawConfig};

pub fn draw(ctx: &mut impl RenderContext, element: &Element, config: &DrawConfig) {
    let mut options = OptionsBuilder::default();
    let options = default_options_generator(element, element.roundness.is_some(), &mut options)
        .fill(Srgba::new(0.0, 0.0, 0.0, 0.0))
        .build()
        .unwrap();
    let default_points = vec![Point::default(), Point::default()];
    let points = match &element.points {
        Some(points) => points,
        None => &default_points,
    };
    let generator = KurboGenerator::new(options);
    let p = get_points2d(points);

    let shape = if element.roundness.is_none() {
        generator.linear_path(&p[..], false)
    } else {
        generator.curve(&p[..])
    };
    let _ = ctx.save();
    ctx.transform(kurbo::Affine::translate((
        (element.x + config.offset_x) as f64,
        (element.y + config.offset_y) as f64,
    )));
    shape.draw(ctx);
    let _ = ctx.restore();
}
