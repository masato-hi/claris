use yaml_rust::Yaml;

use self::super::super::ext::YamlExt;
use self::super::color::Color;
use self::super::error::NodeError;
use self::super::scale::Scale;
use self::super::stroke::Stroke;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
  pub fill: bool,
  pub color: Color,
  pub stroke: Stroke,
  pub scale: Scale,
  pub radius: f64,
}

impl Rectangle {
  pub fn parse(src: &Yaml) -> Result<Rectangle, NodeError> {
    let x = src.f64_val("x").ok_or(NodeError::Required(
      "rectangle".to_string(),
      "x".to_string(),
    ))?;
    let y = src.f64_val("y").ok_or(NodeError::Required(
      "rectangle".to_string(),
      "y".to_string(),
    ))?;
    let width = src.f64_val("width").ok_or(NodeError::Required(
      "rectangle".to_string(),
      "width".to_string(),
    ))?;
    let height = src.f64_val("height").ok_or(NodeError::Required(
      "rectangle".to_string(),
      "height".to_string(),
    ))?;
    let fill = src.bool_val("fill").unwrap_or(false);
    let radius = src.f64_val("radius").unwrap_or(0.0);
    let alpha = src.f32_val("alpha").unwrap_or(255.0);
    let color = src
      .string_val("color")
      .ok_or(NodeError::Required(
        "rectangle".to_string(),
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

    Ok(Rectangle {
      x: x,
      y: y,
      width: width,
      height: height,
      fill: fill,
      color: color,
      stroke: stroke,
      scale: scale,
      radius: radius,
    })
  }
}

#[cfg(test)]
mod parse_tests {
  extern crate cairo;
  extern crate yaml_rust;

  use super::Rectangle;
  use cairo::LineCap;
  use yaml_rust::YamlLoader;

  #[test]
  fn x_is_integer() {
    let s = "
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let src = &docs[0];
    let rectangle = Rectangle::parse(&src).unwrap();
    assert_eq!(rectangle.x, 10.0);
  }

  #[test]
  fn x_is_float() {
    let s = "
x: 20.6
y: 20
width: 30
height: 40
color: '#AABBCC'
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let src = &docs[0];
    let rectangle = Rectangle::parse(&src).unwrap();
    assert_eq!(rectangle.x, 20.6);
  }

  #[test]
  fn x_is_blank() {
    let s = "
y: 20
width: 30
height: 40
color: '#AABBCC'
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let src = &docs[0];
    assert!(Rectangle::parse(&src).is_err());
  }

  #[test]
  fn color_is_rgb() {
    let s = "
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let src = &docs[0];
    let rectangle = Rectangle::parse(&src).unwrap();
    assert_eq!(rectangle.color.r, 170);
    assert_eq!(rectangle.color.g, 187);
    assert_eq!(rectangle.color.b, 204);
    assert_eq!(rectangle.color.a, 1.0);
  }

  #[test]
  fn color_is_rgba() {
    let s = "
x: 10
y: 20
width: 30
height: 40
color: rgba(255, 128, 64, 0.5)
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let src = &docs[0];
    let rectangle = Rectangle::parse(&src).unwrap();
    assert_eq!(rectangle.color.r, 255);
    assert_eq!(rectangle.color.g, 128);
    assert_eq!(rectangle.color.b, 64);
    assert_eq!(rectangle.color.a, 0.5);
  }

  #[test]
  fn color_and_alpha() {
    let s = "
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
alpha: 0.5
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let src = &docs[0];
    let rectangle = Rectangle::parse(&src).unwrap();
    assert_eq!(rectangle.color.r, 170);
    assert_eq!(rectangle.color.g, 187);
    assert_eq!(rectangle.color.b, 204);
    assert_eq!(rectangle.color.a, 0.5);
  }

  #[test]
  fn stroke_with_integer_width() {
    let s = "
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
stroke:
  width: 2
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let src = &docs[0];
    let rectangle = Rectangle::parse(&src).unwrap();
    assert_eq!(rectangle.stroke.width, 2.0);
    assert_eq!(rectangle.stroke.cap, LineCap::Butt);
  }

  #[test]
  fn stroke_with_float_width() {
    let s = "
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
stroke:
  width: 2.5
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let src = &docs[0];
    let rectangle = Rectangle::parse(&src).unwrap();
    assert_eq!(rectangle.stroke.width, 2.5);
    assert_eq!(rectangle.stroke.cap, LineCap::Butt);
  }

  #[test]
  fn stroke_with_cap() {
    let s = "
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
stroke:
  cap: round
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let src = &docs[0];
    let rectangle = Rectangle::parse(&src).unwrap();
    assert_eq!(rectangle.stroke.width, 1.0);
    assert_eq!(rectangle.stroke.cap, LineCap::Round);
  }
}
