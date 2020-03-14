use log::debug;
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

use self::super::super::ext::YamlExt;
use self::super::color::Color;
use self::super::error::NodeError;
use self::super::Arc;
use self::super::Circle;
use self::super::Curve;
use self::super::Layer;
use self::super::Line;
use self::super::Polygon;
use self::super::Rectangle;
use self::super::Text;
use self::super::Triangle;

#[derive(Debug, Clone)]
pub struct Root {
    pub width: i32,
    pub height: i32,
    pub color: Color,
    pub layers: Vec<Layer>,
}

impl Root {
    pub fn parse(src: &Yaml) -> Result<Root, NodeError> {
        let width = src.i32_val("width").ok_or(NodeError::Required(
            "root node".to_string(),
            "width".to_string(),
        ))?;
        let height = src.i32_val("height").ok_or(NodeError::Required(
            "root node".to_string(),
            "height".to_string(),
        ))?;
        let color = src
            .string_val("color")
            .and_then(|x| -> Option<Color> {
                Color::parse(x)
                    .and_then(|c| -> Result<Color, _> { Ok(Color::new(c.r, c.g, c.b, 1.0)) })
                    .ok()
            })
            .unwrap_or(Color::new(0, 0, 0, 0.0));

        let layers = Self::parse_layers(src)?;

        Ok(Root {
            width: width,
            height: height,
            color: color,
            layers: layers,
        })
    }

    fn parse_layers(src: &Yaml) -> Result<Vec<Layer>, NodeError> {
        let mut ret = Vec::new();

        let layers = src.array_val("layers").ok_or(NodeError::Required(
            "root node".to_string(),
            "layers".to_string(),
        ))?;

        for layer in layers {
            match layer {
                Yaml::Hash(x) => match Self::parse_layer(x) {
                    Ok(x) => ret.push(x),
                    Err(x) => debug!("{:?}", x),
                },
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
