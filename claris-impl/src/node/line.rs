use yaml_rust::Yaml;

use crate::ext::YamlExt;
use crate::node::color::Color;
use crate::node::error::NodeError;
use crate::node::point::{DefPoint, Point};
use crate::node::scale::Scale;
use crate::node::stroke::Stroke;

#[derive(Debug, Clone)]
pub struct Line {
    pub color: Color,
    pub stroke: Stroke,
    pub scale: Scale,
    pub points: Vec<Point>,
}

impl Line {
    pub fn parse(src: &Yaml) -> Result<Line, NodeError> {
        let alpha = src.f32_val("alpha").unwrap_or(255.0);
        let color = src
            .string_val("color")
            .ok_or_else(|| NodeError::Required("line".to_string(), "color".to_string()))
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

        let points = Self::parse_points(src)?;

        Ok(Line {
            color,
            stroke,
            scale,
            points,
        })
    }

    fn parse_points(src: &Yaml) -> Result<Vec<Point>, NodeError> {
        let v = src
            .array_val("points")
            .ok_or_else(|| NodeError::Required("line".to_string(), "points".to_string()))?;

        let mut points = Vec::new();

        for p in v {
            let point = p.as_point().ok_or_else(|| NodeError::InvalidPoint)?;
            points.push(point);
        }

        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::parse_yaml;
    use cairo::LineCap;
    use float_cmp::approx_eq;
    use yaml_rust::YamlLoader;

    macro_rules! parse {
        ($x:expr) => {{
            let src = parse_yaml!($x);
            match Line::parse(&src) {
                Ok(x) => x,
                Err(e) => panic!(e.to_string()),
            }
        }};
    }

    #[test]
    #[should_panic(expected = "'line' is required 'color' option")]
    fn color_is_blank() {
        let s = "---
points:
  - [10, 20]
  - [30, 10]
  - [40, 30]
  - [10, 40]
";
        parse!(s);
    }

    #[test]
    fn color_is_rgb() {
        let s = "---
color: '#AABBCC'
points:
  - [10, 20]
  - [30, 10]
  - [40, 30]
  - [10, 40]
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
color: '#AABBCC'
alpha: 0.5
points:
  - [10, 20]
  - [30, 10]
  - [40, 30]
  - [10, 40]
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
color: '#AABBCC'
points:
  - [10, 20]
  - [30, 10]
  - [40, 30]
  - [10, 40]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.stroke.width, 1.0));
        assert_eq!(subject.stroke.cap, LineCap::Butt);
    }

    #[test]
    fn stroke_with_integer_width() {
        let s = "---
color: '#AABBCC'
stroke:
  width: 2
points:
  - [10, 20]
  - [30, 10]
  - [40, 30]
  - [10, 40]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.stroke.width, 2.0));
        assert_eq!(subject.stroke.cap, LineCap::Butt);
    }

    #[test]
    fn stroke_with_float_width() {
        let s = "---
color: '#AABBCC'
stroke:
  width: 2.5
points:
  - [10, 20]
  - [30, 10]
  - [40, 30]
  - [10, 40]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.stroke.width, 2.5));
        assert_eq!(subject.stroke.cap, LineCap::Butt);
    }

    #[test]
    fn stroke_with_cap() {
        let s = "---
color: '#AABBCC'
stroke:
  cap: round
points:
  - [10, 20]
  - [30, 10]
  - [40, 30]
  - [10, 40]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.stroke.width, 1.0));
        assert_eq!(subject.stroke.cap, LineCap::Round);
    }

    #[test]
    fn scale_is_blank() {
        let s = "---
color: '#AABBCC'
points:
  - [10, 20]
  - [30, 10]
  - [40, 30]
  - [10, 40]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.scale.x, 1.0));
        assert!(approx_eq!(f64, subject.scale.y, 1.0));
    }

    #[test]
    fn scale_is_not_blank() {
        let s = "---
color: '#AABBCC'
scale:
  x: 2
  y: 2.5
points:
  - [10, 20]
  - [30, 10]
  - [40, 30]
  - [10, 40]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.scale.x, 2.0));
        assert!(approx_eq!(f64, subject.scale.y, 2.5));
    }

    #[test]
    #[should_panic(expected = "'line' is required 'points' option")]
    fn points_is_blank() {
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
        parse!(s);
    }

    #[test]
    fn points_is_not_blank() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
scale:
  x: 2
  y: 2.5
points:
  - [10, 20]
  - [30, 10]
  - [40, 30]
  - [10, 40]
";
        let subject = parse!(s);
        assert_eq!(subject.points.len(), 4);
        assert!(approx_eq!(f64, subject.points[0].x, 10.0));
        assert!(approx_eq!(f64, subject.points[0].y, 20.0));
        assert!(approx_eq!(f64, subject.points[1].x, 30.0));
        assert!(approx_eq!(f64, subject.points[1].y, 10.0));
        assert!(approx_eq!(f64, subject.points[2].x, 40.0));
        assert!(approx_eq!(f64, subject.points[2].y, 30.0));
        assert!(approx_eq!(f64, subject.points[3].x, 10.0));
        assert!(approx_eq!(f64, subject.points[3].y, 40.0));
    }

    #[test]
    #[should_panic(expected = "invalid point")]
    fn points_is_invalid() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
scale:
  x: 2
  y: 2.5
points:
  - [10]
";
        parse!(s);
    }
}
