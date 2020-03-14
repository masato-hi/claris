use yaml_rust::Yaml;

use crate::ext::YamlExt;
use crate::node::color::Color;
use crate::node::error::NodeError;
use crate::node::scale::Scale;
use crate::node::stroke::Stroke;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub fill: bool,
    pub color: Color,
    pub stroke: Stroke,
    pub scale: Scale,
    pub radius: f64,
}

impl Circle {
    pub fn parse(src: &Yaml) -> Result<Circle, NodeError> {
        let x = src
            .f64_val("x")
            .ok_or_else(|| NodeError::Required("circle".to_string(), "x".to_string()))?;
        let y = src
            .f64_val("y")
            .ok_or_else(|| NodeError::Required("circle".to_string(), "y".to_string()))?;
        let fill = src.bool_val("fill").unwrap_or(false);
        let radius = src.f64_val("radius").unwrap_or(0.0);
        let alpha = src.f32_val("alpha").unwrap_or(1.0);
        let color = src
            .string_val("color")
            .ok_or_else(|| NodeError::Required("circle".to_string(), "color".to_string()))
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

        Ok(Circle {
            x,
            y,
            fill,
            color,
            stroke,
            scale,
            radius,
        })
    }
}
