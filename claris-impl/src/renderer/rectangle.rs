use crate::node::Rectangle as Node;
use cairo::Context;

pub struct Rectangle {}

impl Rectangle {
    pub fn render(context: &mut Context, node: Node) {
        context.translate(node.x, node.y);
        context.set_source_rgba(
            node.color.r as f64 / 255.0,
            node.color.g as f64 / 255.0,
            node.color.b as f64 / 255.0,
            node.color.a as f64,
        );
        context.scale(node.scale.x, node.scale.y);
        context.rectangle(0.0, 0.0, node.width, node.height);
        if node.fill {
            context.fill();
        } else {
            context.set_line_width(node.stroke.width);
            context.set_line_cap(node.stroke.cap);
            context.stroke();
        }
    }
}
