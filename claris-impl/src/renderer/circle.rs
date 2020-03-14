use crate::node::Circle as Node;
use cairo::Context;
use std::f64::consts::PI;

pub struct Circle {}

impl Circle {
    pub fn render(context: &mut Context, node: Node) {
        context.translate(node.x, node.y);
        context.set_source_rgba(
            node.color.r as f64 / 255.0,
            node.color.g as f64 / 255.0,
            node.color.b as f64 / 255.0,
            node.color.a as f64,
        );
        context.scale(node.scale.x, node.scale.y);
        context.arc(
            0.0,
            0.0,
            node.radius,
            0.0 * (PI / 180.0),
            360.0 * (PI / 180.0),
        );
        if node.fill {
            context.fill();
        } else {
            context.set_line_width(node.stroke.width);
            context.set_line_cap(node.stroke.cap);
            context.stroke();
        }
    }
}
