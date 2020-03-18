use crate::node::Color;
use crate::node::Point;
use crate::node::Polygon;
use crate::node::Scale;
use crate::node::Stroke;

impl Default for Polygon {
    fn default() -> Polygon {
        Polygon {
            fill: false,
            color: Color::default(),
            scale: Scale::default(),
            stroke: Stroke::default(),
            vertex: vec![Point::default(), Point::default(), Point::default()],
        }
    }
}
