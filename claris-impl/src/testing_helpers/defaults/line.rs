use crate::node::Color;
use crate::node::Line;
use crate::node::Point;
use crate::node::Scale;
use crate::node::Stroke;

impl Default for Line {
    fn default() -> Line {
        Line {
            color: Color::default(),
            scale: Scale::default(),
            stroke: Stroke::default(),
            points: vec![Point::default(), Point::default(), Point::default()],
        }
    }
}
