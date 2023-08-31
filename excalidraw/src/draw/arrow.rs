use std::f64::consts::PI;

use super::utils::{get_points2d, hypot, srgba_from_hex};
use super::DrawConfig;
use crate::element::Arrowhead;
use crate::point::Point;
use crate::{draw::utils::default_options_generator, element::Element};
use palette::Srgba;
use piet::kurbo::{BezPath, PathEl};
use piet::{kurbo, RenderContext};
use rough_piet::{KurboDrawable, KurboGenerator};
use roughr::core::{FillStyle, OptionsBuilder};

pub fn draw(ctx: &mut impl RenderContext, element: &Element, config: &DrawConfig) {
    let mut options = OptionsBuilder::default();
    let options = default_options_generator(element, element.roundness.is_some(), &mut options);

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
    let mut shapes: Vec<KurboDrawable<f64>> = vec![];

    let generator = KurboGenerator::new(
        options
            .clone()
            .fill(Srgba::new(0.0, 0.0, 0.0, 0.0))
            .build()
            .unwrap(),
    );
    let p = get_points2d(points);

    let shape = if element.roundness.is_none() {
        generator.linear_path(&p[..], false)
    } else {
        generator.curve(&p[..])
    };

    match &element.start_arrowhead {
        Some(start_arrowhead) => match start_arrowhead {
            Arrowhead::Arrow => {
                let (point1, point2, point3) = get_arrow_arrowhead_point(&shape, element, true);
                shapes
                    .push(generator.linear_path(&[point2.to_point2d(), point1.to_point2d()], true));
                shapes
                    .push(generator.linear_path(&[point3.to_point2d(), point1.to_point2d()], true));
            }
            Arrowhead::Bar => {
                let (point1, point2, point3) = get_bar_arrowhead_point(&shape, element, true);
                shapes
                    .push(generator.linear_path(&[point2.to_point2d(), point1.to_point2d()], true));
                shapes
                    .push(generator.linear_path(&[point3.to_point2d(), point1.to_point2d()], true));
            }
            Arrowhead::Dot => {
                let default_color = Srgba::new(0.0, 0.0, 0.0, 0.0);
                let stroke_color =
                    srgba_from_hex(&element.stroke_color, element.opacity).unwrap_or(default_color);
                let generator = KurboGenerator::new(
                    options
                        .clone()
                        .fill(stroke_color)
                        .fill_style(FillStyle::Solid)
                        .build()
                        .unwrap(),
                );
                let (point, r) = get_dot_arrowhead_point(&shape, element, true);
                shapes.push(generator.circle(point.x, point.y, r));
            }
            Arrowhead::Triangle => {
                let default_color = Srgba::new(0.0, 0.0, 0.0, 0.0);
                let stroke_color =
                    srgba_from_hex(&element.stroke_color, element.opacity).unwrap_or(default_color);
                let generator = KurboGenerator::new(
                    options
                        .clone()
                        .fill(stroke_color)
                        .fill_style(FillStyle::Solid)
                        .stroke_line_dash([].to_vec())
                        .build()
                        .unwrap(),
                );
                let (point1, point2, point3) = get_triangle_arrowhead_point(&shape, element, true);
                let p = vec![
                    point1.to_point2d(),
                    point2.to_point2d(),
                    point3.to_point2d(),
                    point1.to_point2d(),
                ];
                shapes.push(generator.polygon(&p[..]));
            }
        },
        None => {}
    }

    match &element.end_arrowhead {
        Some(end_arrowhead) => match end_arrowhead {
            Arrowhead::Arrow => {
                let (point1, point2, point3) = get_arrow_arrowhead_point(&shape, element, false);
                shapes
                    .push(generator.linear_path(&[point2.to_point2d(), point1.to_point2d()], true));
                shapes
                    .push(generator.linear_path(&[point3.to_point2d(), point1.to_point2d()], true));
            }
            Arrowhead::Bar => {
                let (point1, point2, point3) = get_bar_arrowhead_point(&shape, element, false);
                shapes
                    .push(generator.linear_path(&[point2.to_point2d(), point1.to_point2d()], true));
                shapes
                    .push(generator.linear_path(&[point3.to_point2d(), point1.to_point2d()], true));
            }
            Arrowhead::Dot => {
                let default_color = Srgba::new(0.0, 0.0, 0.0, 0.0);
                let stroke_color =
                    srgba_from_hex(&element.stroke_color, element.opacity).unwrap_or(default_color);
                let generator = KurboGenerator::new(
                    options
                        .clone()
                        .fill(stroke_color)
                        .fill_style(FillStyle::Solid)
                        .build()
                        .unwrap(),
                );
                let (point, r) = get_dot_arrowhead_point(&shape, element, false);
                shapes.push(generator.circle(point.x, point.y, r));
            }
            Arrowhead::Triangle => {
                let default_color = Srgba::new(0.0, 0.0, 0.0, 0.0);
                let stroke_color =
                    srgba_from_hex(&element.stroke_color, element.opacity).unwrap_or(default_color);
                let generator = KurboGenerator::new(
                    options
                        .clone()
                        .fill(stroke_color)
                        .fill_style(FillStyle::Solid)
                        .stroke_line_dash([].to_vec())
                        .build()
                        .unwrap(),
                );
                let (point1, point2, point3) = get_triangle_arrowhead_point(&shape, element, false);
                let p = vec![
                    point1.to_point2d(),
                    point2.to_point2d(),
                    point3.to_point2d(),
                    point1.to_point2d(),
                ];
                shapes.push(generator.polygon(&p[..]));
            }
        },
        None => {}
    }

    shapes.push(shape);

    shapes.iter().for_each(|s| s.draw(ctx));
    let _ = ctx.restore();
}

fn get_curve_path_ops(shape: &KurboDrawable<f64>) -> &BezPath {
    let sets = &shape.sets;
    for set in sets {
        if set.op_set_type == roughr::core::OpSetType::Path {
            return &set.ops;
        }
    }
    &sets[0].ops
}

fn get_start_and_end_point(ops: &BezPath, is_start: bool) -> (Point, Point) {
    let elements = ops.elements();
    let index = if is_start { 1 } else { elements.len() - 1 };
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

    let end_point = Point::new(x1, y1);
    let start_point = Point::new(x2, y2);
    (start_point, end_point)
}

fn get_arrow_point(start_point: Point, end_point: Point, length: f64, size: f64) -> Point {
    let x2 = start_point.x;
    let y2 = start_point.y;
    let x1 = end_point.x;
    let y1 = end_point.y;

    let distance = hypot(x2 - x1, y2 - y1);
    let nx = (x2 - x1) / distance;
    let ny = (y2 - y1) / distance;

    let min_size = size.min(length / 2.0);
    let xs = x2 - nx * min_size;
    let ys = y2 - ny * min_size;
    Point::new(xs, ys)
}

/**
 * 生成箭头数据
 */
fn get_arrow_arrowhead_point(
    shape: &KurboDrawable<f64>,
    element: &Element,
    is_start: bool,
) -> (Point, Point, Point) {
    let ops = get_curve_path_ops(shape);
    let (start_point, end_point) = get_start_and_end_point(ops, is_start);

    let mut length = 0.0;

    // arrowhead === arrow
    // Length for -> arrows is based on the length of the last section
    if let Some(points) = &element.points {
        let point1 = points.last().unwrap();
        let mut point2 = Point::default();
        if points.len() > 1 {
            point2 = points[points.len() - 2].clone();
        }
        length = hypot(point1.x - point2.x, point1.y - point2.y);
    }
    let arrow_point = get_arrow_point(start_point, end_point, length, 30.0);

    let angle = 20.0;
    let point1 = rotate(arrow_point, start_point, (-angle * PI) / 180.0);
    let point2 = rotate(arrow_point, start_point, (angle * PI) / 180.0);
    (start_point, point1, point2)
}

fn get_bar_arrowhead_point(
    shape: &KurboDrawable<f64>,
    element: &Element,
    is_start: bool,
) -> (Point, Point, Point) {
    let ops = get_curve_path_ops(shape);
    let (start_point, end_point) = get_start_and_end_point(ops, is_start);

    let mut length = 0.0;

    // Length for other arrowhead types is based on the total length of the line
    if let Some(points) = &element.points {
        for (index, point) in points.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let prev_point = &points[index - 1];
            length += hypot(point.x - prev_point.x, point.y - prev_point.y);
        }
    }
    let arrow_point = get_arrow_point(start_point, end_point, length, 15.0);

    let angle = 90.0;
    let point1 = rotate(arrow_point, start_point, (-angle * PI) / 180.0);
    let point2 = rotate(arrow_point, start_point, (angle * PI) / 180.0);
    (start_point, point1, point2)
}

fn get_dot_arrowhead_point(
    shape: &KurboDrawable<f64>,
    element: &Element,
    is_start: bool,
) -> (Point, f64) {
    let ops = get_curve_path_ops(shape);
    let (start_point, end_point) = get_start_and_end_point(ops, is_start);
    let mut length = 0.0;

    // Length for other arrowhead types is based on the total length of the line
    if let Some(points) = &element.points {
        for (index, point) in points.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let prev_point = &points[index - 1];
            length += hypot(point.x - prev_point.x, point.y - prev_point.y);
        }
    }
    let arrow_point = get_arrow_point(start_point, end_point, length, 15.0);
    let r = hypot(arrow_point.y - start_point.y, arrow_point.x - start_point.x)
        + element.stroke_width as f64;
    (start_point, r)
}

fn get_triangle_arrowhead_point(
    shape: &KurboDrawable<f64>,
    element: &Element,
    is_start: bool,
) -> (Point, Point, Point) {
    let ops = get_curve_path_ops(shape);
    let (start_point, end_point) = get_start_and_end_point(ops, is_start);

    let mut length = 0.0;

    // Length for other arrowhead types is based on the total length of the line
    if let Some(points) = &element.points {
        for (index, point) in points.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let prev_point = &points[index - 1];
            length += hypot(point.x - prev_point.x, point.y - prev_point.y);
        }
    }
    let arrow_point = get_arrow_point(start_point, end_point, length, 15.0);

    let angle = 25.0;
    let point1 = rotate(arrow_point, start_point, (-angle * PI) / 180.0);
    let point2 = rotate(arrow_point, start_point, (angle * PI) / 180.0);
    (start_point, point1, point2)
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

fn rotate(point1: Point, point2: Point, angle: f64) -> Point {
    let x = (point1.x - point2.x) * angle.cos() - (point1.y - point2.y) * angle.sin() + point2.x;
    let y = (point1.x - point2.x) * angle.sin() + (point1.y - point2.y) * angle.cos() + point2.y;
    Point::new(x, y)
}
