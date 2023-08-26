use crate::element::Element;
use log::debug;
use palette::Srgba;
use piet::RenderContext;
use rough_piet::KurboGenerator;
use roughr::core::OptionsBuilder;
use serde::{de, Deserialize, Serialize};

fn draw_rectangle(ctx: &mut impl RenderContext, element: &Element, config: &DrawConfig) {
    let default_color = Srgba::new(0.0, 0.0, 0.0, 255.0);
    let stroke_color = srgba_from_hex(&element.stroke_color).unwrap_or(default_color);
    let fill_color = srgba_from_hex(&element.background_color).unwrap_or(default_color);
    let options = OptionsBuilder::default()
        .stroke(stroke_color)
        .fill(fill_color)
        .fill_style(element.fill_style.into_roughr())
        .fill_weight(element.stroke_width as f32)
        .build()
        .unwrap();
    let generator = KurboGenerator::new(options);
    debug!("element: {:?}", element);
    debug!("config: {:?}", config);
    let circle_paths = generator.rectangle::<f32>(
        element.x + config.offset_x,
        element.y + config.offset_y,
        element.width,
        element.height,
    );
    circle_paths.draw(ctx);
}

pub fn draw(ctx: &mut impl RenderContext, elements: &Vec<Element>, config: &DrawConfig) {
    for element in elements {
        draw_rectangle(ctx, element, config);
    }
}

fn srgba_from_hex(hex: &str) -> Option<Srgba> {
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
    let a = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).ok()?
    } else {
        255 // Default alpha value
    };

    // Create an Srgba object
    Some(Srgba::new(r, g, b, a).into_format())
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawConfig {
    pub offset_x: f32,
    pub offset_y: f32,
}
