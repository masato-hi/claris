use crate::node::triangle::Vertex;
use crate::node::Color;
use crate::node::Point;
use crate::node::Scale;
use crate::node::Stroke;
use crate::node::Triangle;

impl Default for Triangle {
    fn default() -> Self {
        Self {
            fill: false,
            color: Color::default(),
            scale: Scale::default(),
            stroke: Stroke::default(),
            vertex: Vertex::default(),
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            a: Point::default(),
            b: Point::default(),
            c: Point::default(),
        }
    }
}
