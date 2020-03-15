use yaml_rust::Yaml;

use crate::ext::YamlExt;
use crate::node::color::Color;
use crate::node::error::NodeError;
use crate::node::point::{DefPoint, Point};
use crate::node::scale::Scale;
use crate::node::stroke::Stroke;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub fill: bool,
    pub color: Color,
    pub stroke: Stroke,
    pub scale: Scale,
    pub vertex: Vertex,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn parse(src: &Yaml) -> Result<Triangle, NodeError> {
        let fill = src.bool_val("fill").unwrap_or(false);
        let alpha = src.f32_val("alpha").unwrap_or(1.0);
        let color = src
            .string_val("color")
            .ok_or_else(|| NodeError::Required("triangle".to_string(), "color".to_string()))
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

        let vertex = Self::parse_vertex(src)?;

        Ok(Triangle {
            fill,
            color,
            stroke,
            scale,
            vertex,
        })
    }

    fn parse_vertex(src: &Yaml) -> Result<Vertex, NodeError> {
        let v = src
            .array_val("vertex")
            .ok_or_else(|| NodeError::Required("triangle".to_string(), "vertex".to_string()))?;

        if v.len() != 3 {
            return Err(NodeError::InvalidVertex);
        }

        let a = v[0].as_point().ok_or_else(|| NodeError::InvalidPoint)?;
        let b = v[1].as_point().ok_or_else(|| NodeError::InvalidPoint)?;
        let c = v[2].as_point().ok_or_else(|| NodeError::InvalidPoint)?;

        Ok(Vertex { a, b, c })
    }
}

#[cfg(test)]
mod tests {
    use super::Triangle;
    use crate::parse_yaml;
    use cairo::LineCap;
    use float_cmp::approx_eq;
    use yaml_rust::YamlLoader;

    macro_rules! parse {
        ($x:expr) => {{
            let src = parse_yaml!($x);
            match Triangle::parse(&src) {
                Ok(x) => x,
                Err(e) => panic!(e.to_string()),
            }
        }};
    }

    #[test]
    fn fill_is_true() {
        let s = "---
color: '#AABBCC'
fill: true
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
";
        let subject = parse!(s);
        assert_eq!(subject.fill, true)
    }

    #[test]
    fn fill_is_false() {
        let s = "---
color: '#AABBCC'
fill: false
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
";
        let subject = parse!(s);
        assert_eq!(subject.fill, false)
    }

    #[test]
    fn fill_is_blank() {
        let s = "---
color: '#AABBCC'
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
";
        let subject = parse!(s);
        assert_eq!(subject.fill, false)
    }

    #[test]
    #[should_panic(expected = "'triangle' is required 'color' option")]
    fn color_is_blank() {
        let s = "---
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
";
        parse!(s);
    }

    #[test]
    fn color_is_rgb() {
        let s = "---
color: '#AABBCC'
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
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
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
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
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
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
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
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
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
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
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.stroke.width, 1.0));
        assert_eq!(subject.stroke.cap, LineCap::Round);
    }

    #[test]
    fn scale_is_blank() {
        let s = "---
color: '#AABBCC'
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
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
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.scale.x, 2.0));
        assert!(approx_eq!(f64, subject.scale.y, 2.5));
    }

    #[test]
    #[should_panic(expected = "'triangle' is required 'vertex' option")]
    fn vertex_is_blank() {
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
    fn vertex_is_not_blank() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
scale:
  x: 2
  y: 2.5
vertex:
  - [10, 20]
  - [30, 10]
  - [40, 30]
";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.vertex.a.x, 10.0));
        assert!(approx_eq!(f64, subject.vertex.a.y, 20.0));
        assert!(approx_eq!(f64, subject.vertex.b.x, 30.0));
        assert!(approx_eq!(f64, subject.vertex.b.y, 10.0));
        assert!(approx_eq!(f64, subject.vertex.c.x, 40.0));
        assert!(approx_eq!(f64, subject.vertex.c.y, 30.0));
    }

    #[test]
    #[should_panic(expected = "invalid vertex")]
    fn vertex_is_invalid() {
        let s = "---
x: 10
y: 20
width: 30
height: 40
color: '#AABBCC'
scale:
  x: 2
  y: 2.5
vertex:
  - [10, 20]
";
        parse!(s);
    }
}
