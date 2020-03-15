use cairo::FontSlant;
use cairo::FontWeight;
use yaml_rust::Yaml;

use crate::ext::YamlExt;
use crate::node::color::Color;
use crate::node::error::NodeError;
use crate::node::scale::Scale;

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
            .ok_or_else(|| NodeError::Required("text".to_string(), "x".to_string()))?;
        let y = src
            .f64_val("y")
            .ok_or_else(|| NodeError::Required("text".to_string(), "y".to_string()))?;
        let alpha = src.f32_val("alpha").unwrap_or(1.0);
        let color = src
            .string_val("color")
            .ok_or_else(|| NodeError::Required("text".to_string(), "color".to_string()))
            .and_then(|x| -> Result<Color, NodeError> {
                Color::parse(x).and_then(|c| -> Result<Color, NodeError> {
                    Ok(Color::new(c.r, c.g, c.b, alpha))
                })
            })?;
        let scale = src
            .hash_val("scale")
            .map_or(Scale::default(), |x| -> Scale { Scale::parse(x) });
        let text = src
            .string_val("text")
            .ok_or_else(|| NodeError::Required("text".to_string(), "text".to_string()))?;
        let family = src
            .string_val("family")
            .unwrap_or_else(|| Self::DEFAULT_FAMILY.to_string());
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
            x,
            y,
            color,
            scale,
            text,
            size,
            family,
            weight,
            slant,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Text;
    use crate::parse_yaml;
    use cairo::{FontSlant, FontWeight};
    use float_cmp::approx_eq;
    use yaml_rust::YamlLoader;

    macro_rules! parse {
        ($x:expr) => {{
            let src = parse_yaml!($x);
            match Text::parse(&src) {
                Ok(x) => x,
                Err(e) => panic!(e.to_string()),
            }
        }};
    }

    #[test]
    fn parse_integer_x_y() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.x, 10.0));
        assert!(approx_eq!(f64, subject.y, 20.0));
    }

    #[test]
    fn parse_float_x_y() {
        let s = "---
x: 10.1
y: 20.2
color: '#AABBCC'
text: test
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.x, 10.1));
        assert!(approx_eq!(f64, subject.y, 20.2));
    }

    #[test]
    #[should_panic(expected = "'text' is required 'x' option")]
    fn x_is_blank() {
        let s = "---
y: 20
width: 30
height: 40
color: '#AABBCC'
text: test
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "'text' is required 'y' option")]
    fn y_is_blank() {
        let s = "---
x: 10
width: 30
height: 40
color: '#AABBCC'
text: test
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "'text' is required 'color' option")]
    fn color_is_blank() {
        let s = "---
x: 10
y: 20
text: test
";
        parse!(s);
    }

    #[test]
    fn color_is_rgb() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
";
        let subject = parse!(s);
        assert_eq!(subject.color.r, 170);
        assert_eq!(subject.color.g, 187);
        assert_eq!(subject.color.b, 204);
        assert!(approx_eq!(f32, subject.color.a, 1.0));
    }

    #[test]
    fn color_and_alpha() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
alpha: 0.5
text: test
";
        let subject = parse!(s);
        assert_eq!(subject.color.r, 170);
        assert_eq!(subject.color.g, 187);
        assert_eq!(subject.color.b, 204);
        assert!(approx_eq!(f32, subject.color.a, 0.5));
    }

    #[test]
    fn scale_is_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.scale.x, 1.0));
        assert!(approx_eq!(f64, subject.scale.y, 1.0));
    }

    #[test]
    fn scale_is_not_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
scale:
  x: 2
  y: 2.5
text: test
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.scale.x, 2.0));
        assert!(approx_eq!(f64, subject.scale.y, 2.5));
    }

    #[test]
    fn text_is_not_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
";
        let subject = parse!(s);
        assert_eq!(subject.text, "test");
    }

    #[test]
    #[should_panic(expected = "'text' is required 'text' option")]
    fn text_is_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
";
        parse!(s);
    }

    #[test]
    fn family_is_not_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
family: sans-serif
";
        let subject = parse!(s);
        assert_eq!(subject.family, "sans-serif");
    }

    #[test]
    fn family_is_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
";
        let subject = parse!(s);
        assert_eq!(subject.family, "serif");
    }

    #[test]
    fn size_is_not_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
size: 20
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.size, 20.0));
    }

    #[test]
    fn size_is_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.size, 14.0));
    }

    #[test]
    fn weight_is_normal() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
weight: normal
";
        let subject = parse!(s);
        assert_eq!(subject.weight, FontWeight::Normal);
    }

    #[test]
    fn weight_is_bold() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
weight: bold
";
        let subject = parse!(s);
        assert_eq!(subject.weight, FontWeight::Bold);
    }

    #[test]
    fn weight_is_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
";
        let subject = parse!(s);
        assert_eq!(subject.weight, FontWeight::Normal);
    }

    #[test]
    fn slant_is_normal() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
slant: normal
";
        let subject = parse!(s);
        assert_eq!(subject.slant, FontSlant::Normal);
    }

    #[test]
    fn slant_is_italic() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
slant: italic
";
        let subject = parse!(s);
        assert_eq!(subject.slant, FontSlant::Italic);
    }

    #[test]
    fn slant_is_oblique() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
slant: oblique
";
        let subject = parse!(s);
        assert_eq!(subject.slant, FontSlant::Oblique);
    }

    #[test]
    fn slant_is_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
text: test
";
        let subject = parse!(s);
        assert_eq!(subject.slant, FontSlant::Normal);
    }
}
