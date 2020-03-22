use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

use crate::ext::YamlExt;
use crate::node::color::Color;
use crate::node::error::NodeError;
use crate::node::Arc;
use crate::node::Circle;
use crate::node::Curve;
use crate::node::Layer;
use crate::node::Line;
use crate::node::Polygon;
use crate::node::Rectangle;
use crate::node::Text;
use crate::node::Triangle;

#[derive(Debug, Clone)]
pub struct Root {
    pub width: i32,
    pub height: i32,
    pub color: Color,
    pub layers: Vec<Layer>,
}

impl Root {
    pub fn parse(src: &Yaml) -> Result<Root, NodeError> {
        let width = src
            .i32_val("width")
            .ok_or_else(|| NodeError::Required("root node".to_string(), "width".to_string()))?;
        let height = src
            .i32_val("height")
            .ok_or_else(|| NodeError::Required("root node".to_string(), "height".to_string()))?;
        let color = src
            .string_val("color")
            .and_then(|x| -> Option<Color> {
                Color::parse(x)
                    .and_then(|c| -> Result<Color, _> { Ok(Color::new(c.r, c.g, c.b, 1.0)) })
                    .ok()
            })
            .unwrap_or_else(|| Color::new(0, 0, 0, 0.0));

        let layers = Self::parse_layers(src)?;

        Ok(Root {
            width,
            height,
            color,
            layers,
        })
    }

    fn parse_layers(src: &Yaml) -> Result<Vec<Layer>, NodeError> {
        let mut ret = Vec::new();

        let layers = src
            .array_val("layers")
            .ok_or_else(|| NodeError::Required("root node".to_string(), "layers".to_string()))?;

        for layer in layers {
            match layer {
                Yaml::Hash(x) => Self::parse_layer(x).and_then(|x| -> Result<(), NodeError> {
                    ret.push(x);
                    Ok(())
                })?,
                _ => return Err(NodeError::InvalidLayer),
            }
        }

        Ok(ret)
    }

    fn parse_layer(src: &Hash) -> Result<Layer, NodeError> {
        if src.len() != 1 {
            return Err(NodeError::InvalidLayerCount);
        }

        let key = match src.keys().next() {
            Some(x) => match x.as_str() {
                Some(x) => x,
                _ => return Err(NodeError::InvalidLayerDefine),
            },
            _ => return Err(NodeError::InvalidLayerDefine),
        };

        let entry = src.entry(key).ok_or(NodeError::InvalidLayerDefine)?;

        let ret = match key {
            "rectangle" => {
                let rectangle = Rectangle::parse(entry)?;
                Layer::Rectangle(rectangle)
            }
            "circle" => {
                let circle = Circle::parse(entry)?;
                Layer::Circle(circle)
            }
            "arc" => {
                let arc = Arc::parse(entry)?;
                Layer::Arc(arc)
            }
            "triangle" => {
                let triangle = Triangle::parse(entry)?;
                Layer::Triangle(triangle)
            }
            "polygon" => {
                let polygon = Polygon::parse(entry)?;
                Layer::Polygon(polygon)
            }
            "line" => {
                let line = Line::parse(entry)?;
                Layer::Line(line)
            }
            "curve" => {
                let curve = Curve::parse(entry)?;
                Layer::Curve(curve)
            }
            "text" => {
                let text = Text::parse(entry)?;
                Layer::Text(text)
            }
            _ => return Err(NodeError::UnknownLayer(key.to_string())),
        };

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::Root;
    use crate::node::Layer;
    use crate::parse_yaml;
    use float_cmp::approx_eq;
    use yaml_rust::YamlLoader;

    macro_rules! parse {
        ($x:expr) => {{
            let src = parse_yaml!($x);
            match Root::parse(&src) {
                Ok(x) => x,
                Err(e) => panic!(e.to_string()),
            }
        }};
    }

    #[test]
    #[should_panic(expected = "'root node' is required 'width' option")]
    fn width_is_blank() {
        let s = "---
height: 300
layers:
- line:
    color: '#AABBCC'
    points:
      - [10, 20]
        ";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "'root node' is required 'height' option")]
    fn height_is_blank() {
        let s = "---
width: 400
layers:
- line:
    color: '#AABBCC'
    points:
      - [10, 20]
        ";
        parse!(s);
    }

    #[test]
    fn color_is_blank() {
        let s = "---
width: 400
height: 300
layers:
- line:
    color: '#AABBCC'
    points:
      - [10, 20]
";
        let subject = parse!(s);
        assert_eq!(subject.color.r, 0);
        assert_eq!(subject.color.g, 0);
        assert_eq!(subject.color.b, 0);
        assert!(approx_eq!(f32, subject.color.a, 0.0));
    }

    #[test]
    fn color_is_rgb() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
- line:
    color: '#AABBCC'
    points:
      - [10, 20]
";
        let subject = parse!(s);
        assert_eq!(subject.color.r, 170);
        assert_eq!(subject.color.g, 187);
        assert_eq!(subject.color.b, 204);
        assert!(approx_eq!(f32, subject.color.a, 1.0));
    }

    #[test]
    #[should_panic(expected = "'root node' is required 'layers' option")]
    fn has_invalid_layers() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers: 1
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "invalid layer")]
    fn has_invalid_layer() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - 1
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "invalid layers count")]
    fn has_invalid_layer_count() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - {}
  - {}
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "invalid layer define")]
    fn has_invalid_layer_define() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - 1: 1
";
        parse!(s);
    }

    #[test]
    #[should_panic(expected = "unknown layer type 'test'")]
    fn has_unknown_layer() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - test: {}
";
        parse!(s);
    }

    #[test]
    fn has_arc_layer() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - arc:
      x: 10
      y: 10
      color: '#AABBCC'
      radius: 10
      start: 20
      end: 340
";
        let subject = parse!(s);
        assert_eq!(subject.layers.len(), 1);
        assert!(matches!(subject.layers[0], Layer::Arc(_)));
    }

    #[test]
    fn has_circle_layer() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - circle:
      x: 10
      y: 10
      color: '#AABBCC'
      radius: 10
";
        let subject = parse!(s);
        assert_eq!(subject.layers.len(), 1);
        assert!(matches!(subject.layers[0], Layer::Circle(_)));
    }

    #[test]
    fn has_curve_layer() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - curve:
      color: '#AABBCC'
      start: [10, 10]
      mid: [30, 30]
      end: [10, 60]
";
        let subject = parse!(s);
        assert_eq!(subject.layers.len(), 1);
        assert!(matches!(subject.layers[0], Layer::Curve(_)));
    }

    #[test]
    fn has_line_layer() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - line:
      color: '#AABBCC'
      points:
        - [10, 10]
        - [30, 30]
        - [10, 60]
";
        let subject = parse!(s);
        assert_eq!(subject.layers.len(), 1);
        assert!(matches!(subject.layers[0], Layer::Line(_)));
    }

    #[test]
    fn has_polygon_layer() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - polygon:
      color: '#AABBCC'
      vertex:
        - [10, 10]
        - [30, 30]
        - [10, 60]
";
        let subject = parse!(s);
        assert_eq!(subject.layers.len(), 1);
        assert!(matches!(subject.layers[0], Layer::Polygon(_)));
    }

    #[test]
    fn has_rectangle_layer() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - rectangle:
      x: 10
      y: 10
      width: 200
      height: 100
      color: '#AABBCC'
";
        let subject = parse!(s);
        assert_eq!(subject.layers.len(), 1);
        assert!(matches!(subject.layers[0], Layer::Rectangle(_)));
    }

    #[test]
    fn has_text_layer() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - text:
      x: 10
      y: 10
      text: test
      color: '#AABBCC'
";
        let subject = parse!(s);
        assert_eq!(subject.layers.len(), 1);
        assert!(matches!(subject.layers[0], Layer::Text(_)));
    }

    #[test]
    fn has_triangle_layer() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - triangle:
      color: '#AABBCC'
      vertex:
        - [10, 10]
        - [30, 30]
        - [10, 60]
";
        let subject = parse!(s);
        assert_eq!(subject.layers.len(), 1);
        assert!(matches!(subject.layers[0], Layer::Triangle(_)));
    }

    #[test]
    fn has_many_layer() {
        let s = "---
width: 400
height: 300
color: '#AABBCC'
layers:
  - rectangle:
      x: 10
      y: 10
      width: 200
      height: 100
      color: '#AABBCC'
  - text:
      x: 10
      y: 10
      text: test
      color: '#AABBCC'
  - triangle:
      color: '#AABBCC'
      vertex:
        - [10, 10]
        - [30, 30]
        - [10, 60]
";
        let subject = parse!(s);
        assert_eq!(subject.layers.len(), 3);
        assert!(matches!(subject.layers[0], Layer::Rectangle(_)));
        assert!(matches!(subject.layers[1], Layer::Text(_)));
        assert!(matches!(subject.layers[2], Layer::Triangle(_)));
    }
}
