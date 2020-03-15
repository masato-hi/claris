use crate::node::error::NodeError;
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
            r,
            g,
            b,
            a: if a >= 0.0 && a <= 1.0 {
                a
            } else {
                Self::DEFAULT_ALPHA
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    use float_cmp::approx_eq;

    #[test]
    fn valid_color_format() {
        let subject = Color::parse("#AABBCC".to_string()).unwrap();
        assert_eq!(subject.r, 170);
        assert_eq!(subject.g, 187);
        assert_eq!(subject.b, 204);
        assert!(approx_eq!(f32, subject.a, 1.0));
    }

    #[test]
    #[should_panic(expected = "invalid color format '#AABBCG'")]
    fn invalid_color_format() {
        match Color::parse("#AABBCG".to_string()) {
            Err(e) => panic!(e.to_string()),
            _ => (),
        }
    }
}
