mod draw;
mod element;
mod point;
use draw::DrawConfig;
use element::Element;

use piet::RenderContext;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Excalidraw {
    #[serde(rename = "type")]
    pub type_field: String,
    pub version: i64,
    pub source: String,
    pub elements: Vec<Element>,
    pub app_state: AppState,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub view_background_color: String,
}

impl Excalidraw {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn draw(&self, ctx: &mut impl RenderContext, padding: f32) {
        let rect = self.get_canvas_size();
        draw::draw(
            ctx,
            &self.elements,
            &DrawConfig {
                offset_x: -rect.x + padding,
                offset_y: -rect.y + padding,
            },
        );
    }
    /**
     * 获取画布大小（所有 elements 的外接矩形）
     */
    pub fn get_canvas_size(&self) -> Rect {
        let mut min_x = std::f32::MAX;
        let mut min_y = std::f32::MAX;
        let mut max_x = 0.0;
        let mut max_y = 0.0;
        for element in &self.elements {
            if element.x < min_x {
                min_x = element.x;
            }
            if element.y < min_y {
                min_y = element.y;
            }
            if element.x + element.width > max_x {
                max_x = element.x + element.width;
            }
            if element.y + element.height > max_y {
                max_y = element.y + element.height;
            }
        }
        Rect {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file = std::fs::read_to_string("excalidraw.json").unwrap();
        let excalidraw = Excalidraw::from_json(&file);
        assert!(excalidraw.is_ok())
    }
}
