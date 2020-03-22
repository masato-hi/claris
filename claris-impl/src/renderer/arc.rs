use crate::node::Arc as Node;
use crate::Context;
use std::f64::consts::PI;

pub struct Arc {}

impl Arc {
    pub fn render(context: &mut dyn Context, node: Node) {
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

#[cfg(test)]
mod tests {
    use super::Arc;
    use crate::node::Arc as Node;
    use crate::testing_helpers::stub::ContextImpl;

    #[test]
    fn fill_and_close() {
        let mut context = ContextImpl::new();
        let node = Node {
            fill: true,
            close: true,
            ..Default::default()
        };
        Arc::render(&mut context, node);
        assert_eq!(context.translate_received, 1);
        assert_eq!(context.set_source_rgba_received, 1);
        assert_eq!(context.scale_received, 1);
        assert_eq!(context.move_to_received, 1);
        assert_eq!(context.line_to_received, 1);
        assert_eq!(context.arc_received, 1);
        assert_eq!(context.fill_received, 1);
        assert_eq!(context.set_line_width_received, 0);
        assert_eq!(context.set_line_cap_received, 0);
        assert_eq!(context.stroke_received, 0);
    }

    #[test]
    fn fill_and_not_close() {
        let mut context = ContextImpl::new();
        let node = Node {
            fill: true,
            close: false,
            ..Default::default()
        };
        Arc::render(&mut context, node);
        assert_eq!(context.translate_received, 1);
        assert_eq!(context.set_source_rgba_received, 1);
        assert_eq!(context.scale_received, 1);
        assert_eq!(context.move_to_received, 0);
        assert_eq!(context.line_to_received, 0);
        assert_eq!(context.arc_received, 1);
        assert_eq!(context.fill_received, 1);
        assert_eq!(context.set_line_width_received, 0);
        assert_eq!(context.set_line_cap_received, 0);
        assert_eq!(context.stroke_received, 0);
    }

    #[test]
    fn stroke_and_close() {
        let mut context = ContextImpl::new();
        let node = Node {
            fill: false,
            close: true,
            ..Default::default()
        };
        Arc::render(&mut context, node);
        assert_eq!(context.translate_received, 1);
        assert_eq!(context.set_source_rgba_received, 1);
        assert_eq!(context.scale_received, 1);
        assert_eq!(context.move_to_received, 1);
        assert_eq!(context.line_to_received, 1);
        assert_eq!(context.arc_received, 1);
        assert_eq!(context.fill_received, 0);
        assert_eq!(context.set_line_width_received, 1);
        assert_eq!(context.set_line_cap_received, 1);
        assert_eq!(context.stroke_received, 1);
    }

    #[test]
    fn stroke_and_not_close() {
        let mut context = ContextImpl::new();
        let node = Node {
            fill: false,
            close: false,
            ..Default::default()
        };
        Arc::render(&mut context, node);
        assert_eq!(context.translate_received, 1);
        assert_eq!(context.set_source_rgba_received, 1);
        assert_eq!(context.scale_received, 1);
        assert_eq!(context.move_to_received, 0);
        assert_eq!(context.line_to_received, 0);
        assert_eq!(context.arc_received, 1);
        assert_eq!(context.fill_received, 0);
        assert_eq!(context.set_line_width_received, 1);
        assert_eq!(context.set_line_cap_received, 1);
        assert_eq!(context.stroke_received, 1);
    }
}
