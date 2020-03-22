use crate::node::Color;
use crate::node::Scale;
use crate::node::Text;
use cairo::FontSlant;
use cairo::FontWeight;

impl Default for Text {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            color: Color::default(),
            scale: Scale::default(),
            family: "serif".to_string(),
            weight: FontWeight::Normal,
            slant: FontSlant::Normal,
            size: 14.0,
            text: "Hello World!".to_string(),
        }
    }
}
