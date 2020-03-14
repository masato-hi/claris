use yaml_rust::Yaml;

use crate::ext::YamlExt;
use crate::node::color::Color;
use crate::node::error::NodeError;
use crate::node::point::{DefPoint, Point};
use crate::node::scale::Scale;
use crate::node::stroke::Stroke;

#[derive(Debug, Clone)]
pub struct Line {
    pub color: Color,
    pub stroke: Stroke,
    pub scale: Scale,
    pub points: Vec<Point>,
}

impl Line {
    pub fn parse(src: &Yaml) -> Result<Line, NodeError> {
        let alpha = src.f32_val("alpha").unwrap_or(255.0);
        let color = src
            .string_val("color")
            .ok_or_else(|| NodeError::Required("line".to_string(), "color".to_string()))
            .and_then(|x| -> Result<Color, NodeError> {
                Color::parse(x).and_then(|c| -> Result<Color, NodeError> {
                    Ok(Color::new(c.r, c.g, c.b, alpha))
                })
            })?;
        let stroke = src
            .hash_val("stroke")
            .map_or(Stroke::default(), |x| -> Stroke { Stroke::parse(x) });
        let scale = src
            .hash_val("scale")
            .map_or(Scale::default(), |x| -> Scale { Scale::parse(x) });

        let points = Self::parse_points(src)?;

        Ok(Line {
            color,
            stroke,
            scale,
            points,
        })
    }

    fn parse_points(src: &Yaml) -> Result<Vec<Point>, NodeError> {
        let v = src
            .array_val("points")
            .ok_or_else(|| NodeError::Required("line".to_string(), "points".to_string()))?;

        let mut points = Vec::new();

        for p in v {
            let point = p.as_point().ok_or_else(|| NodeError::InvalidPoint)?;
            points.push(point);
        }

        Ok(points)
    }
}
