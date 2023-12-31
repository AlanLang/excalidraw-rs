mod arrow;
mod diamond;
mod ellipse;
mod line;
mod rectangle;
mod utils;
use crate::element::{Element, ElementType};
use piet::RenderContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawConfig {
    pub offset_x: f32,
    pub offset_y: f32,
}

pub fn draw(ctx: &mut impl RenderContext, elements: &Vec<Element>, config: &DrawConfig) {
    for element in elements {
        if element.is_deleted {
            continue;
        }
        match element.element_type {
            ElementType::Rectangle => rectangle::draw(ctx, element, config),
            ElementType::Diamond => diamond::draw(ctx, element, config),
            ElementType::Ellipse => ellipse::draw(ctx, element, config),
            ElementType::Arrow => arrow::draw(ctx, element, config),
            ElementType::Line => line::draw(ctx, element, config),
            _ => {}
        }
    }
}
