use crate::node::Triangle as Node;
use crate::Context;

pub struct Triangle {}

impl Triangle {
    pub fn render(context: &mut dyn Context, node: Node) {
        context.translate(0.0, 0.0);
        context.set_source_rgba(
            node.color.r as f64 / 255.0,
            node.color.g as f64 / 255.0,
            node.color.b as f64 / 255.0,
            node.color.a as f64,
        );
        context.scale(node.scale.x, node.scale.y);
        context.move_to(node.vertex.a.x, node.vertex.a.y);
        context.line_to(node.vertex.b.x, node.vertex.b.y);
        context.line_to(node.vertex.c.x, node.vertex.c.y);
        context.close_path();
        if node.fill {
            context.fill();
        } else {
            context.set_line_width(node.stroke.width);
            context.set_line_cap(node.stroke.cap);
            context.stroke();
        }
    }
}
