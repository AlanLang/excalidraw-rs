use std::f64::consts::PI;

use super::DrawConfig;
use crate::point::Point;
use crate::{draw::utils::default_options_generator, element::Element};
use euclid::UnknownUnit;
use log::debug;
use palette::Srgba;
use piet::kurbo::{BezPath, PathEl};
use piet::{kurbo, RenderContext};
use rough_piet::{KurboDrawable, KurboGenerator};
use roughr::core::OptionsBuilder;
use roughr::Point2D;

pub fn draw(ctx: &mut impl RenderContext, element: &Element, config: &DrawConfig) {
    let mut options = OptionsBuilder::default();
    let options = default_options_generator(element, element.roundness.is_some(), &mut options)
        .fill(Srgba::new(0.0, 0.0, 0.0, 0.0))
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
        let generator = KurboGenerator::new(options.clone());
        let p = get_points2d(points);
        // https://github.com/excalidraw/excalidraw/pull/737/files
        shape.push(generator.linear_path(&p[..], false));
    } else {
        let generator = KurboGenerator::new(options.clone());
        let p = get_points2d(points);
        let a = generator.curve(&p[..]);
        shape.push(generator.curve(&p[..]));
        let (x2, y2, x3, y3, x4, y4) = demo(&a, element);
        shape.push(generator.linear_path(&[Point2D::new(x3, y3), Point2D::new(x2, y2)], true));
        shape.push(generator.linear_path(&[Point2D::new(x4, y4), Point2D::new(x2, y2)], true));
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

pub fn hypot<T>(a: T, b: T) -> f64
where
    T: core::ops::Mul<T, Output = T>
        + core::ops::Add<T, Output = T>
        + core::convert::Into<f64>
        + Copy,
{
    ((a * a + b * b).into()).sqrt()
}

fn get_curve_path_ops(shape: &KurboDrawable<f64>) -> &BezPath {
    let sets = &shape.sets;
    for set in sets {
        if set.op_set_type == roughr::core::OpSetType::Path {
            return &set.ops;
        }
        debug!("set {:?}", set);
    }
    &sets[0].ops
}

fn get_start_and_end_point(
    ops: &BezPath,
    is_start: bool,
) -> (piet::kurbo::Point, piet::kurbo::Point) {
    let elements = ops.elements();
    let index = if is_start { 0 } else { elements.len() - 1 };
    let data = &elements[index];
    let binding = piet::kurbo::Point::default();
    let (p1, p2, p3) = match data {
        PathEl::CurveTo(p1, p2, p3) => (p1, p2, p3),
        _ => (&binding, &binding, &binding),
    };
    let mut p0 = piet::kurbo::Point::new(0.0, 0.0);
    let prev_op = &ops.elements()[index - 1];
    match prev_op {
        PathEl::MoveTo(p) => {
            p0 = p.clone();
        }
        PathEl::CurveTo(_, _, p3) => {
            p0 = p3.clone();
        }
        _ => {}
    }
    let (x1, y1) = equation(0.3, &p0, p1, p2, p3);
    let x2 = if is_start { p0.x } else { p3.x };
    let y2 = if is_start { p0.y } else { p3.y };

    let end_point = piet::kurbo::Point::new(x1, y1);
    let start_point = piet::kurbo::Point::new(x2, y2);
    (start_point, end_point)
}

fn get_arrow_point(
    start_point: piet::kurbo::Point,
    end_point: piet::kurbo::Point,
    length: f64,
) -> piet::kurbo::Point {
    let x2 = start_point.x;
    let y2 = start_point.y;
    let x1 = end_point.x;
    let y1 = end_point.y;

    let distance = hypot(x2 - x1, y2 - y1);
    let nx = (x2 - x1) / distance;
    let ny = (y2 - y1) / distance;

    let size: f64 = 30.0;
    let min_size = size.min(length / 2.0);
    let xs = x2 - nx * min_size;
    let ys = y2 - ny * min_size;
    piet::kurbo::Point::new(xs, ys)
}

fn demo(shape: &KurboDrawable<f64>, element: &Element) -> (f64, f64, f64, f64, f64, f64) {
    let ops = get_curve_path_ops(shape);
    let index = 1;
    let data = &ops.elements()[index];
    let binding = piet::kurbo::Point::default();
    let (p1, p2, p3) = match data {
        PathEl::CurveTo(p1, p2, p3) => (p1, p2, p3),
        _ => (&binding, &binding, &binding),
    };

    let mut p0 = piet::kurbo::Point::new(0.0, 0.0);
    let prev_op = &ops.elements()[index - 1];
    match prev_op {
        PathEl::MoveTo(p) => {
            p0 = p.clone();
        }
        PathEl::CurveTo(_, _, p3) => {
            p0 = p3.clone();
        }
        _ => {}
    }
    let (x1, y1) = equation(0.3, &p0, p1, p2, p3);

    let x2 = p0.x;
    let y2 = p0.y;

    let distance = hypot(x2 - x1, y2 - y1);
    let nx = (x2 - x1) / distance;
    let ny = (y2 - y1) / distance;

    let size: f64 = 30.0;
    let mut length = 0.0;

    // arrowhead === arrow
    if let Some(points) = &element.points {
        let point1 = points.last().unwrap();
        let mut point2 = Point::default();
        if points.len() > 1 {
            point2 = points[points.len() - 2].clone();
        }
        length = hypot(point1.x - point2.x, point1.y - point2.y);
    }
    let min_size = size.min(length / 2.0);
    let xs = x2 - nx * min_size;
    let ys = y2 - ny * min_size;

    let angle = 20.0;
    let (x3, y3) = rotate(xs, ys, x2, y2, (-angle * PI) / 180.0);
    let (x4, y4) = rotate(xs, ys, x2, y2, (angle * PI) / 180.0);
    debug!("x1 {:?} y1 {:?}", x1, y1);
    debug!("x2 {:?} y2 {:?}", x2, y2);
    debug!("x3 {:?} y3 {:?}", x3, y3);
    debug!("x4 {:?} y4 {:?}", x4, y4);
    (x2, y2, x3, y3, x4, y4)
}

fn equation(
    t: f64,
    p0: &piet::kurbo::Point,
    p1: &piet::kurbo::Point,
    p2: &piet::kurbo::Point,
    p3: &piet::kurbo::Point,
) -> (f64, f64) {
    let x = (1.0 - t).powf(3.0) * p3.x
        + 3.0 * t * (1.0 - t).powf(2.0) * p2.x
        + 3.0 * t.powf(2.0) * (1.0 - t) * p1.x
        + p0.x * t.powf(3.0);

    let y = (1.0 - t).powf(3.0) * p3.y
        + 3.0 * t * (1.0 - t).powf(2.0) * p2.y
        + 3.0 * t.powf(2.0) * (1.0 - t) * p1.y
        + p0.y * t.powf(3.0);
    (x, y)
}

fn rotate(x1: f64, y1: f64, x2: f64, y2: f64, angle: f64) -> (f64, f64) {
    (
        (x1 - x2) * angle.cos() - (y1 - y2) * angle.sin() + x2,
        (x1 - x2) * angle.sin() + (y1 - y2) * angle.cos() + y2,
    )
}
