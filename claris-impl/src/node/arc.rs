use yaml_rust::Yaml;

use self::super::super::ext::YamlExt;
use self::super::color::Color;
use self::super::error::NodeError;
use self::super::scale::Scale;
use self::super::stroke::Stroke;

#[derive(Debug, Clone, Copy)]
pub struct Arc {
  pub x: f64,
  pub y: f64,
  pub start: f64,
  pub end: f64,
  pub close: bool,
  pub fill: bool,
  pub color: Color,
  pub stroke: Stroke,
  pub scale: Scale,
  pub radius: f64,
}

impl Arc {
  pub fn parse(src: &Yaml) -> Result<Arc, NodeError> {
    let x = src
      .f64_val("x")
      .ok_or(NodeError::Required("circle".to_string(), "x".to_string()))?;
    let y = src
      .f64_val("y")
      .ok_or(NodeError::Required("circle".to_string(), "y".to_string()))?;
    let start = src.f64_val("start").ok_or(NodeError::Required(
      "circle".to_string(),
      "start".to_string(),
    ))?;
    let end = src
      .f64_val("end")
      .ok_or(NodeError::Required("circle".to_string(), "end".to_string()))?;
    let fill = src.bool_val("fill").unwrap_or(false);
    let close = src.bool_val("close").unwrap_or(false);
    let radius = src.f64_val("radius").unwrap_or(0.0);
    let alpha = src.f32_val("alpha").unwrap_or(255.0);
    let color = src
      .string_val("color")
      .ok_or(NodeError::Required(
        "circle".to_string(),
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

    Ok(Arc {
      x: x,
      y: y,
      start: start,
      end: end,
      fill: fill,
      close: close,
      color: color,
      stroke: stroke,
      scale: scale,
      radius: radius,
    })
  }
}
