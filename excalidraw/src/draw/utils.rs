use crate::element::{Element, Roundness, RoundnessType, StrokeStyle};
use log::debug;
use palette::Srgba;
use roughr::core::OptionsBuilder;

pub fn default_options_generator<'a, 'b>(
    element: &'a Element,
    continuous_path: bool,
    options: &'b mut OptionsBuilder,
) -> &'b mut OptionsBuilder {
    let default_color = Srgba::new(0.0, 0.0, 0.0, 0.0);
    let stroke_color =
        srgba_from_hex(&element.stroke_color, element.opacity).unwrap_or(default_color);
    let fill_color =
        srgba_from_hex(&element.background_color, element.opacity).unwrap_or(default_color);
    options
        .seed(element.seed)
        .fill_style(element.fill_style.into_roughr())
        .stroke_width(get_stroke_width(
            &element.stroke_style,
            element.stroke_width,
        ))
        .stroke_line_dash(stroke_line_dash(
            &element.stroke_style,
            element.stroke_width,
        ))
        .fill_weight(element.stroke_width / 2 as f32)
        .hachure_gap(element.stroke_width * 4 as f32)
        .disable_multi_stroke(element.stroke_style != StrokeStyle::Solid)
        .roughness(element.roughness)
        .stroke(stroke_color)
        .fill(fill_color)
        .preserve_vertices(continuous_path)
        .line_cap(roughr::core::LineCap::Round)
        .line_join(roughr::core::LineJoin::Round);

    options
}

fn srgba_from_hex(hex: &str, opacity: u8) -> Option<Srgba> {
    // Remove the leading '#' if it exists
    let hex = hex.trim_start_matches('#');

    // Check the length of our string
    if hex.len() != 6 && hex.len() != 8 {
        return None;
    }

    // Parse the hex string into RGBA components
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    let mut a = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).ok()?
    } else {
        255 // Default alpha value
    };

    // Apply the opacity scaling from 0 to 100 to 0 to 255
    a = (a as f32 * (opacity as f32 / 100.0)) as u8;

    // Create an Srgba object
    Some(Srgba::new(r, g, b, a).into_format())
}

pub fn stroke_line_dash(stroke_style: &StrokeStyle, stroke_width: f32) -> Vec<f64> {
    debug!("stroke_style: {:?}", stroke_style);
    match stroke_style {
        StrokeStyle::Solid => vec![],
        StrokeStyle::Dashed => vec![8 as f64, 8 as f64 + stroke_width as f64],
        StrokeStyle::Dotted => vec![1.5 as f64, 6 as f64 + stroke_width as f64],
    }
}

pub fn get_stroke_width(stroke_style: &StrokeStyle, stroke_width: f32) -> f32 {
    if stroke_style == &StrokeStyle::Solid {
        stroke_width
    } else {
        stroke_width + 0.5 as f32
    }
}

pub fn get_corner_radius(x: f32, roundness: &Roundness) -> f32 {
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
