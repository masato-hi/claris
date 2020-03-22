use crate::node::Color;
use crate::node::Curve;
use crate::node::Point;
use crate::node::Scale;
use crate::node::Stroke;

impl Default for Curve {
    fn default() -> Self {
        Self {
            color: Color::default(),
            scale: Scale::default(),
            stroke: Stroke::default(),
            start: Point::default(),
            mid: Point::default(),
            end: Point::default(),
        }
    }
}
