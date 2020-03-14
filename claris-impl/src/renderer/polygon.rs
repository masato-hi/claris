use cairo::Context;

use crate::node::Polygon as Node;

pub struct Polygon {}

impl Polygon {
    pub fn render(context: &mut Context, node: Node) {
        context.translate(0.0, 0.0);
        context.set_source_rgba(
            node.color.r as f64 / 255.0,
            node.color.g as f64 / 255.0,
            node.color.b as f64 / 255.0,
            node.color.a as f64,
        );
        context.scale(node.scale.x, node.scale.y);
        for (i, point) in node.vertex.iter().enumerate() {
            if i == 0 {
                context.move_to(point.x, point.y)
            } else {
                context.line_to(point.x, point.y)
            }
        }
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
