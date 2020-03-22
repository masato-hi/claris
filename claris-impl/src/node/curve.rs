use yaml_rust::Yaml;

use crate::ext::YamlExt;
use crate::node::color::Color;
use crate::node::error::NodeError;
use crate::node::point::{DefPoint, Point};
use crate::node::scale::Scale;
use crate::node::stroke::Stroke;

#[derive(Debug, Clone)]
pub struct Curve {
    pub color: Color,
    pub stroke: Stroke,
    pub scale: Scale,
    pub start: Point,
    pub mid: Point,
    pub end: Point,
}

impl Curve {
    pub fn parse(src: &Yaml) -> Result<Curve, NodeError> {
        let alpha = src.f32_val("alpha").unwrap_or(1.0);
        let color = src
            .string_val("color")
            .ok_or_else(|| NodeError::Required("curve".to_string(), "color".to_string()))
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

        let start = src
            .array_val("start")
            .ok_or_else(|| NodeError::Required("curve".to_string(), "start".to_string()))
            .and_then(|x| -> Result<Point, NodeError> {
                x.as_point().ok_or(NodeError::InvalidPoint)
            })?;
        let mid = src
            .array_val("mid")
            .ok_or_else(|| NodeError::Required("curve".to_string(), "mid".to_string()))
            .and_then(|x| -> Result<Point, NodeError> {
                x.as_point().ok_or_else(|| NodeError::InvalidPoint)
            })?;
        let end = src
            .array_val("end")
            .ok_or_else(|| NodeError::Required("curve".to_string(), "end".to_string()))
            .and_then(|x| -> Result<Point, NodeError> {
                x.as_point().ok_or_else(|| NodeError::InvalidPoint)
            })?;

        Ok(Curve {
            color,
            stroke,
            scale,
            start,
            mid,
            end,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Curve;
    use crate::parse_yaml;
    use cairo::LineCap;
    use float_cmp::approx_eq;
    use yaml_rust::YamlLoader;

    macro_rules! parse {
        ($x:expr) => {{
            let src = parse_yaml!($x);
            match Curve::parse(&src) {
                Ok(x) => x,
                Err(e) => panic!(e.to_string()),
            }
        }};
    }

    #[test]
    #[should_panic(expected = "'curve' is required 'color' option")]
    fn color_is_blank() {
        let s = "---
start: [10, 5]
mid: [30, 40]
end: [10, 60]
";
        parse!(s);
    }

    #[test]
    fn color_is_rgb() {
        let s = "---
color: '#AABBCC'
start: [10, 5]
mid: [30, 40]
end: [10, 60]
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
start: [10, 5]
mid: [30, 40]
end: [10, 60]
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
start: [10, 5]
mid: [30, 40]
end: [10, 60]
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
start: [10, 5]
mid: [30, 40]
end: [10, 60]
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
start: [10, 5]
mid: [30, 40]
end: [10, 60]
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
start: [10, 5]
mid: [30, 40]
end: [10, 60]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.stroke.width, 1.0));
        assert_eq!(subject.stroke.cap, LineCap::Round);
    }

    #[test]
    fn scale_is_blank() {
        let s = "---
color: '#AABBCC'
start: [10, 5]
mid: [30, 40]
end: [10, 60]
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
start: [10, 5]
mid: [30, 40]
end: [10, 60]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.scale.x, 2.0));
        assert!(approx_eq!(f64, subject.scale.y, 2.5));
    }

    #[test]
    #[should_panic(expected = "'curve' is required 'start' option")]
    fn start_is_blank() {
        let s = "---
color: '#AABBCC'
mid: [30, 40]
end: [10, 60]
";
        parse!(s);
    }

    #[test]
    fn start_is_not_blank() {
        let s = "---
color: '#AABBCC'
start: [10, 5]
mid: [30, 40]
end: [10, 60]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.start.x, 10.0));
        assert!(approx_eq!(f64, subject.start.y, 5.0));
    }

    #[test]
    #[should_panic(expected = "invalid point")]
    fn start_is_invalid() {
        let s = "---
color: '#AABBCC'
start: [10]
mid: [30, 40]
end: [10, 60]
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "'curve' is required 'mid' option")]
    fn mid_is_blank() {
        let s = "---
color: '#AABBCC'
start: [10, 5]
end: [10, 60]
";
        parse!(s);
    }

    #[test]
    fn mid_is_not_blank() {
        let s = "---
color: '#AABBCC'
start: [10, 5]
mid: [30, 40]
end: [10, 60]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.mid.x, 30.0));
        assert!(approx_eq!(f64, subject.mid.y, 40.0));
    }

    #[test]
    #[should_panic(expected = "invalid point")]
    fn mid_is_invalid() {
        let s = "---
color: '#AABBCC'
start: [10, 5]
mid: [30]
end: [10, 60]
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "'curve' is required 'end' option")]
    fn end_is_blank() {
        let s = "---
color: '#AABBCC'
start: [10, 5]
mid: [30, 40]
";
        parse!(s);
    }

    #[test]
    fn end_is_not_blank() {
        let s = "---
color: '#AABBCC'
start: [10, 5]
mid: [30, 40]
end: [10, 60]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.end.x, 10.0));
        assert!(approx_eq!(f64, subject.end.y, 60.0));
    }

    #[test]
    #[should_panic(expected = "invalid point")]
    fn end_is_invalid() {
        let s = "---
color: '#AABBCC'
start: [10, 5]
mid: [30, 40]
end: [10]
";
        parse!(s);
    }
}
