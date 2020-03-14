use self::super::super::node::Curve as Node;
use cairo::Context;

pub struct Curve {}

impl Curve {
    pub fn render(context: &mut Context, node: Node) {
        context.translate(0.0, 0.0);
        context.set_source_rgba(
            node.color.r as f64 / 255.0,
            node.color.g as f64 / 255.0,
            node.color.b as f64 / 255.0,
            node.color.a as f64,
        );
        context.scale(node.scale.x, node.scale.y);
        context.set_line_width(node.stroke.width);
        context.set_line_cap(node.stroke.cap);
        context.curve_to(
            node.start.x,
            node.start.y,
            node.midway.x,
            node.midway.y,
            node.end.x,
            node.end.y,
        );
        context.stroke();
    }
}
