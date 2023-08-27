use crate::{draw::utils::default_options_generator, element::Element};
use log::debug;
use piet::{kurbo, RenderContext};
use rough_piet::KurboGenerator;
use roughr::{core::OptionsBuilder, Point2D};

use super::{utils::get_corner_radius, DrawConfig};

pub fn draw(ctx: &mut impl RenderContext, element: &Element, config: &DrawConfig) {
    let mut options = OptionsBuilder::default();
    let options = default_options_generator(element, element.roundness.is_some(), &mut options)
        .build()
        .unwrap();
    let generator = KurboGenerator::new(options);
    let (top_x, top_y, right_x, right_y, bottom_x, bottom_y, left_x, left_y) =
        get_diamond_points(element);
    let path = match &element.roundness {
        Some(roundness) => {
            let vertical_radius = get_corner_radius((top_x - left_x).abs(), roundness);
            let horizontal_radius = get_corner_radius((right_y - top_y).abs(), roundness);
            let path = format!(
                "M {} {} L {} {} C {} {}, {} {}, {} {} L {} {} C {} {}, {} {}, {} {} L {} {} C {} {}, {} {}, {} {} L {} {} C {} {}, {} {}, {} {}",
                top_x + vertical_radius,
                top_y + horizontal_radius,
                right_x - vertical_radius,
                right_y - horizontal_radius,
                right_x,
                right_y,
                right_x,
                right_y,
                right_x - vertical_radius,
                right_y + horizontal_radius,
                bottom_x + vertical_radius,
                bottom_y - horizontal_radius,
                bottom_x,
                bottom_y,
                bottom_x,
                bottom_y,
                bottom_x - vertical_radius,
                bottom_y - horizontal_radius,
                left_x + vertical_radius,
                left_y + horizontal_radius,
                left_x,
                left_y,
                left_x,
                left_y,
                left_x + vertical_radius,
                left_y - horizontal_radius,
                top_x - vertical_radius,
                top_y + horizontal_radius,
                top_x,
                top_y,
                top_x,
                top_y,
                top_x + vertical_radius,
                top_y + horizontal_radius,
            );
            generator.path::<f32>(path)
        }
        None => {
            let points = [
                Point2D::new(top_x, top_y),
                Point2D::new(right_x, right_y),
                Point2D::new(bottom_x, bottom_y),
                Point2D::new(left_x, left_y),
            ];
            debug!("points: {:?}", points);
            generator.polygon::<f32>(&points)
        }
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
