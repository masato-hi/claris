use crate::node::Polygon as Node;
use crate::Context;

pub struct Polygon {}

impl Polygon {
    pub fn render(context: &mut dyn Context, node: Node) {
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

#[cfg(test)]
mod tests {
    use super::Polygon;
    use crate::node::Polygon as Node;
    use crate::testing_helpers::stub::ContextImpl;

    #[test]
    fn fill_mode() {
        let mut context = ContextImpl::new();
        let node = Node {
            fill: true,
            ..Default::default()
        };
        Polygon::render(&mut context, node);
        assert_eq!(context.translate_received, 1);
        assert_eq!(context.set_source_rgba_received, 1);
        assert_eq!(context.scale_received, 1);
        assert_eq!(context.move_to_received, 1);
        assert_eq!(context.line_to_received, 2);
        assert_eq!(context.close_path_received, 1);
        assert_eq!(context.fill_received, 1);
        assert_eq!(context.set_line_width_received, 0);
        assert_eq!(context.set_line_cap_received, 0);
        assert_eq!(context.stroke_received, 0);
    }

    #[test]
    fn stroke_mode() {
        let mut context = ContextImpl::new();
        let node = Node::default();
        Polygon::render(&mut context, node);
        assert_eq!(context.translate_received, 1);
        assert_eq!(context.set_source_rgba_received, 1);
        assert_eq!(context.scale_received, 1);
        assert_eq!(context.move_to_received, 1);
        assert_eq!(context.line_to_received, 2);
        assert_eq!(context.close_path_received, 1);
        assert_eq!(context.fill_received, 0);
        assert_eq!(context.set_line_width_received, 1);
        assert_eq!(context.set_line_cap_received, 1);
        assert_eq!(context.stroke_received, 1);
    }
}
