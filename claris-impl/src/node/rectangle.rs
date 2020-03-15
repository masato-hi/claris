use yaml_rust::Yaml;

use crate::ext::YamlExt;
use crate::node::color::Color;
use crate::node::error::NodeError;
use crate::node::scale::Scale;
use crate::node::stroke::Stroke;

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
        let x = src
            .f64_val("x")
            .ok_or_else(|| NodeError::Required("rectangle".to_string(), "x".to_string()))?;
        let y = src
            .f64_val("y")
            .ok_or_else(|| NodeError::Required("rectangle".to_string(), "y".to_string()))?;
        let width = src
            .f64_val("width")
            .ok_or_else(|| NodeError::Required("rectangle".to_string(), "width".to_string()))?;
        let height = src
            .f64_val("height")
            .ok_or_else(|| NodeError::Required("rectangle".to_string(), "height".to_string()))?;
        let fill = src.bool_val("fill").unwrap_or(false);
        let radius = src.f64_val("radius").unwrap_or(0.0);
        let alpha = src.f32_val("alpha").unwrap_or(1.0);
        let color = src
            .string_val("color")
            .ok_or_else(|| NodeError::Required("rectangle".to_string(), "color".to_string()))
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

        Ok(Rectangle {
            x,
            y,
            width,
            height,
            fill,
            color,
            stroke,
            scale,
            radius,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;
    use crate::parse_yaml;
    use cairo::LineCap;
    use float_cmp::approx_eq;
    use yaml_rust::YamlLoader;

    macro_rules! parse {
        ($x:expr) => {{
            let src = parse_yaml!($x);
            match Rectangle::parse(&src) {
                Ok(x) => x,
                Err(e) => panic!(e.to_string()),
            }
        }};
    }

    #[test]
    fn parse_integer_x_y_width_height() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.x, 10.0));
        assert!(approx_eq!(f64, subject.y, 20.0));
        assert!(approx_eq!(f64, subject.width, 30.0));
        assert!(approx_eq!(f64, subject.height, 40.0));
    }

    #[test]
    fn parse_float_x_y_width_height() {
        let s = "---
x: 10.1
y: 20.2
width: 30.3
height: 40.4
color: '#AABBCC'
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.x, 10.1));
        assert!(approx_eq!(f64, subject.y, 20.2));
        assert!(approx_eq!(f64, subject.width, 30.3));
        assert!(approx_eq!(f64, subject.height, 40.4));
    }

    #[test]
    #[should_panic(expected = "'rectangle' is required 'x' option")]
    fn x_is_blank() {
        let s = "---
y: 20
width: 30
height: 40
color: '#AABBCC'
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "'rectangle' is required 'y' option")]
    fn y_is_blank() {
        let s = "---
x: 10
width: 30
height: 40
color: '#AABBCC'
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "'rectangle' is required 'width' option")]
    fn width_is_blank() {
        let s = "---
x: 10
y: 20
height: 40
color: '#AABBCC'
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "'rectangle' is required 'height' option")]
    fn height_is_blank() {
        let s = "---
x: 10
y: 20
width: 30
color: '#AABBCC'
";
        parse!(s);
    }

    #[test]
    fn fill_is_true() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
fill: true
";
        let subject = parse!(s);
        assert_eq!(subject.fill, true)
    }

    #[test]
    fn fill_is_false() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
fill: false
";
        let subject = parse!(s);
        assert_eq!(subject.fill, false)
    }

    #[test]
    fn fill_is_blank() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
";
        let subject = parse!(s);
        assert_eq!(subject.fill, false)
    }

    #[test]
    fn radius_is_not_blank() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
radius: 10
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.radius, 10.0));
    }

    #[test]
    fn radius_is_blank() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.radius, 0.0));
    }

    #[test]
    #[should_panic(expected = "'rectangle' is required 'color' option")]
    fn color_is_blank() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
";
        parse!(s);
    }

    #[test]
    fn color_is_rgb() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
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
width: 30
height: 40
color: '#AABBCC'
alpha: 0.5
";
        let subject = parse!(s);
        assert_eq!(subject.color.r, 170);
        assert_eq!(subject.color.g, 187);
        assert_eq!(subject.color.b, 204);
        assert!(approx_eq!(f32, subject.color.a, 0.5));
    }

    #[test]
    fn stroke_is_blank() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.stroke.width, 1.0));
        assert_eq!(subject.stroke.cap, LineCap::Butt);
    }

    #[test]
    fn stroke_with_integer_width() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
stroke:
  width: 2
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.stroke.width, 2.0));
        assert_eq!(subject.stroke.cap, LineCap::Butt);
    }

    #[test]
    fn stroke_with_float_width() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
stroke:
  width: 2.5
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.stroke.width, 2.5));
        assert_eq!(subject.stroke.cap, LineCap::Butt);
    }

    #[test]
    fn stroke_with_cap() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
stroke:
  cap: round
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.stroke.width, 1.0));
        assert_eq!(subject.stroke.cap, LineCap::Round);
    }

    #[test]
    fn scale_is_blank() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
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
width: 30
height: 40
color: '#AABBCC'
scale:
  x: 2
  y: 2.5
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.scale.x, 2.0));
        assert!(approx_eq!(f64, subject.scale.y, 2.5));
    }
}
