use super::DrawConfig;
use crate::element::Arrowhead;
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
            // 绘制曲线
            let generator = KurboGenerator::new(options.clone());
            let p = get_points2d(points);
            let a = generator.linear_path(&p[..], false);
            let points = a.sets[0].size;
            debug!("{:?}", points);
            todo!("根据曲线端点，计算箭头的角度");
            // https://github.com/excalidraw/excalidraw/pull/737/files
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

fn get_start_arrowhead_points(
    element: &Element,
    arrowhead: Arrowhead,
    points: &Vec<Point>,
) -> Vec<Point2D<f64, UnknownUnit>> {
    let mut arrow_points: Vec<Point2D<f64, UnknownUnit>> = vec![];
    if points.len() < 2 {
        return arrow_points;
    }
    let start_point = points[0].clone();
    let second_point = points[1].clone();
    let length = hypot(
        start_point.x - second_point.x,
        start_point.y - second_point.y,
    )
    .min(30.0);
    arrow_points.push(Point2D::new(start_point.x, start_point.y));
    todo!()
}

pub fn hypot<T>(a: T, b: T) -> f64
where
    T: core::ops::Mul<T, Output = T>
        + core::ops::Add<T, Output = T>
        + core::convert::Into<f64>
        + Copy,
{
    ((a * a + b * b).into()).sqrt()
}
