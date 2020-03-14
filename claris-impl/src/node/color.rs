use self::super::error::NodeError;
use css_color_parser::Color as CssColor;

#[derive(Debug, Clone, Copy)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: f32,
}

impl Color {
  const DEFAULT_ALPHA: f32 = 1.0;

  pub fn parse(src: String) -> Result<Color, NodeError> {
    match src.parse::<CssColor>() {
      Ok(x) => Ok(Self::new(x.r, x.g, x.b, x.a)),
      Err(_) => Err(NodeError::InvalidColor(src)),
    }
  }

  pub fn new(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color {
      r: r,
      g: g,
      b: b,
      a: if a >= 0.0 && a <= 1.0 {
        a
      } else {
        Self::DEFAULT_ALPHA
      },
    }
  }
}
