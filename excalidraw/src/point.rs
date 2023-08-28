use std::fmt;

use serde::{
    de::{self, SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.x)?;
        seq.serialize_element(&self.y)?;
        seq.end()
    }
}

struct PointVisitor;

impl<'de> Visitor<'de> for PointVisitor {
    type Value = Point;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence containing two elements")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let x = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let y = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        Ok(Point { x, y })
    }
}

impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Point, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(PointVisitor)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() {
        let json_data = r#"
        {
            "points": [
                [0, 0],
                [217.98046875, 225.5859375]
            ]
        }
        "#;

        #[derive(Debug, Deserialize)]
        struct PointsContainer {
            points: Vec<Point>,
        }

        let deserialized: PointsContainer = serde_json::from_str(json_data).unwrap();
        assert_eq!(deserialized.points[0], Point { x: 0.0, y: 0.0 });
        assert_eq!(
            deserialized.points[1],
            Point {
                x: 217.98046875,
                y: 225.5859375
            }
        );
    }

    #[test]
    fn test_serialization() {
        #[derive(Debug, Serialize)]
        struct PointsContainer {
            points: Vec<Point>,
        }
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point {
                x: 217.98046875,
                y: 225.5859375,
            },
        ];
        let points_container = PointsContainer { points };

        let serialized = serde_json::to_string(&points_container).unwrap();
        let expected_json = r#"{"points":[[0.0,0.0],[217.98046875,225.5859375]]}"#;
        assert_eq!(serialized, expected_json);
    }
}
