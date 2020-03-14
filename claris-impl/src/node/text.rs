use cairo::FontSlant;
use cairo::FontWeight;
use yaml_rust::Yaml;

use self::super::super::ext::YamlExt;
use self::super::color::Color;
use self::super::error::NodeError;
use self::super::scale::Scale;

#[derive(Debug, Clone)]
pub struct Text {
  pub x: f64,
  pub y: f64,
  pub color: Color,
  pub scale: Scale,
  pub text: String,
  pub family: String,
  pub size: f64,
  pub weight: FontWeight,
  pub slant: FontSlant,
}

impl Text {
  const DEFAULT_FAMILY: &'static str = "serif";
  const DEFAULT_SIZE: f64 = 14.0;

  pub fn parse(src: &Yaml) -> Result<Text, NodeError> {
    let x = src
      .f64_val("x")
      .ok_or(NodeError::Required("text".to_string(), "x".to_string()))?;
    let y = src
      .f64_val("y")
      .ok_or(NodeError::Required("text".to_string(), "y".to_string()))?;
    let alpha = src.f32_val("alpha").unwrap_or(1.0);
    let color = src
      .string_val("color")
      .ok_or(NodeError::Required("text".to_string(), "color".to_string()))
      .and_then(|x| -> Result<Color, NodeError> {
        Color::parse(x)
          .and_then(|c| -> Result<Color, NodeError> { Ok(Color::new(c.r, c.g, c.b, alpha)) })
      })?;
    let scale = src
      .hash_val("scale")
      .map_or(Scale::default(), |x| -> Scale { Scale::parse(x) });
    let text = src
      .string_val("text")
      .ok_or(NodeError::Required("text".to_string(), "text".to_string()))?;
    let family = src
      .string_val("family")
      .unwrap_or(Self::DEFAULT_FAMILY.to_string());
    let size = src.f64_val("size").unwrap_or(Self::DEFAULT_SIZE);
    let weight = match src.str_val("weight") {
      Some("bold") => FontWeight::Bold,
      _ => FontWeight::Normal,
    };
    let slant = match src.str_val("slant") {
      Some("italic") => FontSlant::Italic,
      Some("oblique") => FontSlant::Oblique,
      _ => FontSlant::Normal,
    };

    Ok(Text {
      x: x,
      y: y,
      color: color,
      scale: scale,
      text: text,
      size: size,
      family: family,
      weight: weight,
      slant: slant,
    })
  }
}
