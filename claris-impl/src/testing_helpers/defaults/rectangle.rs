use crate::node::Color;
use crate::node::Rectangle;
use crate::node::Scale;
use crate::node::Stroke;

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            fill: false,
            radius: 0.0,
            color: Color::default(),
            scale: Scale::default(),
            stroke: Stroke::default(),
        }
    }
}
