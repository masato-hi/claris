use yaml_rust::Yaml;

use self::super::super::ext::YamlExt;
use self::super::color::Color;
use self::super::error::NodeError;
use self::super::point::{DefPoint, Point};
use self::super::scale::Scale;
use self::super::stroke::Stroke;

#[derive(Debug, Clone)]
pub struct Curve {
  pub color: Color,
  pub stroke: Stroke,
  pub scale: Scale,
  pub start: Point,
  pub midway: Point,
  pub end: Point,
}

impl Curve {
  pub fn parse(src: &Yaml) -> Result<Curve, NodeError> {
    let alpha = src.f32_val("alpha").unwrap_or(255.0);
    let color = src
      .string_val("color")
      .ok_or(NodeError::Required(
        "curve".to_string(),
        "color".to_string(),
      ))
      .and_then(|x| -> Result<Color, NodeError> {
        Color::parse(x)
          .and_then(|c| -> Result<Color, NodeError> { Ok(Color::new(c.r, c.g, c.b, alpha)) })
      })?;
    let stroke = src
      .hash_val("stroke")
      .map_or(Stroke::default(), |x| -> Stroke { Stroke::parse(x) });
    let scale = src
      .hash_val("scale")
      .map_or(Scale::default(), |x| -> Scale { Scale::parse(x) });

    let start = src
      .array_val("start")
      .ok_or(NodeError::Required(
        "curve".to_string(),
        "start".to_string(),
      ))
      .and_then(|x| -> Result<Point, NodeError> { x.as_point().ok_or(NodeError::InvalidPoint) })?;
    let midway = src
      .array_val("midway")
      .ok_or(NodeError::Required(
        "curve".to_string(),
        "midway".to_string(),
      ))
      .and_then(|x| -> Result<Point, NodeError> { x.as_point().ok_or(NodeError::InvalidPoint) })?;
    let end = src
      .array_val("end")
      .ok_or(NodeError::Required("curve".to_string(), "end".to_string()))
      .and_then(|x| -> Result<Point, NodeError> { x.as_point().ok_or(NodeError::InvalidPoint) })?;

    Ok(Curve {
      color: color,
      stroke: stroke,
      scale: scale,
      start: start,
      midway: midway,
      end: end,
    })
  }
}
