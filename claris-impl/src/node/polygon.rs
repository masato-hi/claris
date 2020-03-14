use yaml_rust::Yaml;

use self::super::super::ext::YamlExt;
use self::super::color::Color;
use self::super::error::NodeError;
use self::super::point::{DefPoint, Point};
use self::super::scale::Scale;
use self::super::stroke::Stroke;

#[derive(Debug, Clone)]
pub struct Polygon {
  pub fill: bool,
  pub color: Color,
  pub stroke: Stroke,
  pub scale: Scale,
  pub vertex: Vec<Point>,
}

impl Polygon {
  pub fn parse(src: &Yaml) -> Result<Polygon, NodeError> {
    let fill = src.bool_val("fill").unwrap_or(false);
    let alpha = src.f32_val("alpha").unwrap_or(1.0);
    let color = src
      .string_val("color")
      .ok_or(NodeError::Required(
        "polygon".to_string(),
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

    let vertex = Self::parse_vertex(src)?;

    Ok(Polygon {
      fill: fill,
      color: color,
      stroke: stroke,
      scale: scale,
      vertex: vertex,
    })
  }

  fn parse_vertex(src: &Yaml) -> Result<Vec<Point>, NodeError> {
    let v = src.array_val("vertex").ok_or(NodeError::Required(
      "triangle".to_string(),
      "vertex".to_string(),
    ))?;

    let mut vertex = Vec::new();

    for p in v {
      let point = p.as_point().ok_or(NodeError::InvalidPoint)?;
      vertex.push(point);
    }

    Ok(vertex)
  }
}
