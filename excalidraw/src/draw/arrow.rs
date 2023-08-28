use super::DrawConfig;
use crate::point::Point;
use crate::{draw::utils::default_options_generator, element::Element};
use euclid::UnknownUnit;
use log::debug;
use piet::{kurbo, RenderContext};
use rough_piet::{KurboDrawable, KurboGenerator};
use roughr::core::OptionsBuilder;
use roughr::Point2D;

pub fn draw(ctx: &mut impl RenderContext, element: &Element, config: &DrawConfig) {
    let mut options = OptionsBuilder::default();
    let options = default_options_generator(element, element.roundness.is_some(), &mut options)
        .build()
        .unwrap();
    let _ = ctx.save();
    ctx.transform(kurbo::Affine::translate((
        (element.x + config.offset_x) as f64,
        (element.y + config.offset_y) as f64,
    )));
    let default_points = vec![Point::default(), Point::default()];
    let points = match &element.points {
        Some(points) => points,
        None => &default_points,
    };
    debug!("{:?}", options.fill.is_some());
    let mut shape: Vec<KurboDrawable<f64>> = vec![];
    if element.roundness.is_none() {
        debug!("roundness: {:?}", element.background_color);
        if element.background_color != "transparent" {
            let generator = KurboGenerator::new(options.clone());
            let p = get_points2d(points);
            shape.push(generator.polygon(&p[..]));
        } else {
            let generator = KurboGenerator::new(options.clone());
            let p = get_points2d(points);
            shape.push(generator.linear_path(&p[..], false));
        }
    } else {
        let generator = KurboGenerator::new(options.clone());
        let p = get_points2d(points);
        shape.push(generator.curve(&p[..]));
    }
    // todo get_arrowhead_shapes
    shape.iter().for_each(|s| s.draw(ctx));
    let _ = ctx.restore();
}

fn get_points2d(points: &Vec<Point>) -> Vec<Point2D<f64, UnknownUnit>> {
    points
        .iter()
        .map(|point| Point2D::new(point.x, point.y))
        .collect()
}
