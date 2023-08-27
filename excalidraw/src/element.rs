use log::info;
use palette::Srgba;
use piet::RenderContext;
use rough_piet::KurboGenerator;
use roughr::core::OptionsBuilder;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FillStyle {
    Solid,
    Hachure,
    ZigZag,
    CrossHatch,
    Dots,
    Dashed,
    ZigZagLine,
}
impl Default for FillStyle {
    fn default() -> Self {
        Self::Solid
    }
}
impl FillStyle {
    pub fn into_roughr(&self) -> roughr::core::FillStyle {
        match self {
            Self::Solid => roughr::core::FillStyle::Solid,
            Self::Hachure => roughr::core::FillStyle::Hachure,
            Self::ZigZag => roughr::core::FillStyle::ZigZag,
            Self::CrossHatch => roughr::core::FillStyle::CrossHatch,
            Self::Dots => roughr::core::FillStyle::Dots,
            Self::Dashed => roughr::core::FillStyle::Dashed,
            Self::ZigZagLine => roughr::core::FillStyle::ZigZagLine,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementType {
    Rectangle,
    Diamond,
    Ellipse,
    Arrow,
    Text,
    Selection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StrokeStyle {
    Solid,
    Dashed,
    Dotted,
}
impl Default for StrokeStyle {
    fn default() -> Self {
        Self::Solid
    }
}

impl Default for ElementType {
    fn default() -> Self {
        Self::Rectangle
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RoundnessType {
    Legacy,
    ProportionalRadius,
    AdaptiveRadius,
}

impl<'de> Deserialize<'de> for RoundnessType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let i = u8::deserialize(deserializer)?;
        match i {
            1 => Ok(RoundnessType::Legacy),
            2 => Ok(RoundnessType::ProportionalRadius),
            3 => Ok(RoundnessType::AdaptiveRadius),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Roundness {
    #[serde(rename = "type")]
    pub type_field: RoundnessType,
    pub value: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub id: String,
    #[serde(rename = "type")]
    pub element_type: ElementType,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub angle: i64,
    pub stroke_color: String,
    pub background_color: String,
    pub fill_style: FillStyle,
    pub stroke_width: f32,
    pub stroke_style: StrokeStyle,
    pub roughness: f32,
    pub opacity: u8,
    pub seed: u64,
    pub version: i64,
    pub version_nonce: i64,
    pub is_deleted: bool,
    pub updated: i64,
    pub locked: bool,
    pub roundness: Option<Roundness>,
}

impl Element {
    pub fn draw(&self, ctx: &mut impl RenderContext) {
        info!("Drawing element: {:?}", self);
        let options = OptionsBuilder::default()
            .stroke(Srgba::from_components((114u8, 87u8, 82u8, 255u8)).into_format())
            .fill(Srgba::from_components((254u8, 246u8, 201u8, 255u8)).into_format())
            .fill_style(FillStyle::Hachure.into_roughr())
            .fill_weight(self.stroke_width)
            .build()
            .unwrap();
        let generator = KurboGenerator::new(options);
        let circle_paths =
            generator.rectangle::<f32>(100 as f32, 100 as f32, 100 as f32, 100 as f32);
        circle_paths.draw(ctx);
    }

    pub fn get_size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundness_deserialization() {
        let json_str = r#"{
            "type": 3
        }"#;

        let result: Result<Roundness, serde_json::Error> = serde_json::from_str(json_str);

        match result {
            Ok(roundness) => {
                assert_eq!(roundness.type_field, RoundnessType::AdaptiveRadius);
                assert_eq!(roundness.value, None);
            }
            Err(e) => {
                panic!("Deserialization error: {:?}", e);
            }
        }
    }
}
