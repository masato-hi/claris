use crate::node::Arc as Node;
use cairo::Context;
use std::f64::consts::PI;

pub struct Arc {}

impl Arc {
    pub fn render(context: &mut Context, node: Node) {
        context.translate(node.x, node.y);
        context.set_source_rgba(
            node.color.r as f64 / 255.0,
            node.color.g as f64 / 255.0,
            node.color.b as f64 / 255.0,
            node.color.a as f64,
        );
        context.scale(node.scale.x, node.scale.y);
        if node.close {
            context.move_to(0.0, 0.0)
        }
        context.arc(
            0.0,
            0.0,
            node.radius,
            node.start * (PI / 180.0),
            node.end * (PI / 180.0),
        );
        if node.close {
            context.line_to(0.0, 0.0)
        }
        if node.fill {
            context.fill();
        } else {
            context.set_line_width(node.stroke.width);
            context.set_line_cap(node.stroke.cap);
            context.stroke();
        }
    }
}
