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
        let radius = src
            .f64_val("radius")
            .ok_or_else(|| NodeError::Required("circle".to_string(), "radius".to_string()))?;
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

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::parse_yaml;
    use cairo::LineCap;
    use float_cmp::approx_eq;
    use yaml_rust::YamlLoader;

    macro_rules! parse {
        ($x:expr) => {{
            let src = parse_yaml!($x);
            match Circle::parse(&src) {
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
radius: 15
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
radius: 15
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.x, 10.1));
        assert!(approx_eq!(f64, subject.y, 20.2));
    }

    #[test]
    #[should_panic(expected = "'circle' is required 'x' option")]
    fn x_is_blank() {
        let s = "---
y: 20
color: '#AABBCC'
radius: 15
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "'circle' is required 'y' option")]
    fn y_is_blank() {
        let s = "---
x: 10
color: '#AABBCC'
radius: 15
";
        parse!(s);
    }

    #[test]
    fn fill_is_true() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
fill: true
radius: 15
";
        let subject = parse!(s);
        assert_eq!(subject.fill, true)
    }

    #[test]
    fn fill_is_false() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
fill: false
radius: 15
";
        let subject = parse!(s);
        assert_eq!(subject.fill, false)
    }

    #[test]
    fn fill_is_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
radius: 15
";
        let subject = parse!(s);
        assert_eq!(subject.fill, false)
    }

    #[test]
    fn radius_is_not_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
radius: 15
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.radius, 15.0));
    }

    #[test]
    #[should_panic(expected = "'circle' is required 'radius' option")]
    fn radius_is_blank() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "'circle' is required 'color' option")]
    fn color_is_blank() {
        let s = "---
x: 10
y: 20
radius: 15
";
        parse!(s);
    }

    #[test]
    fn color_is_rgb() {
        let s = "---
x: 10
y: 20
color: '#AABBCC'
radius: 15
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
radius: 15
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
color: '#AABBCC'
radius: 15
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
color: '#AABBCC'
stroke:
  width: 2
radius: 15
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
color: '#AABBCC'
stroke:
  width: 2.5
radius: 15
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
color: '#AABBCC'
stroke:
  cap: round
radius: 15
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
color: '#AABBCC'
radius: 15
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
radius: 15
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.scale.x, 2.0));
        assert!(approx_eq!(f64, subject.scale.y, 2.5));
    }
}
