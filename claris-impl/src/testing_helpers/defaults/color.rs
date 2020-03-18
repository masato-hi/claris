use crate::node::Color;

impl Default for Color {
    fn default() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 1.0,
        }
    }
}
