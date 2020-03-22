use crate::node::Circle;
use crate::node::Color;
use crate::node::Scale;
use crate::node::Stroke;

impl Default for Circle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            fill: false,
            radius: 0.0,
            color: Color::default(),
            scale: Scale::default(),
            stroke: Stroke::default(),
        }
    }
}
