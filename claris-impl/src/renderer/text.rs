use crate::node::Text as Node;
use crate::Context;

pub struct Text {}

impl Text {
    pub fn render(context: &mut dyn Context, node: Node) {
        context.translate(node.x, node.y);
        context.move_to(0.0, 0.0);
        context.set_source_rgba(
            node.color.r as f64 / 255.0,
            node.color.g as f64 / 255.0,
            node.color.b as f64 / 255.0,
            node.color.a as f64,
        );
        context.scale(node.scale.x, node.scale.y);
        context.select_font_face(node.family.as_str(), node.slant, node.weight);
        context.set_font_size(node.size);
        context.show_text(node.text.as_str());
        context.stroke();
    }
}

#[cfg(test)]
mod tests {
    use super::Text;
    use crate::node::Text as Node;
    use crate::testing_helpers::stub::ContextImpl;

    #[test]
    fn render() {
        let mut context = ContextImpl::new();
        let node = Node::default();
        Text::render(&mut context, node);
        assert_eq!(context.translate_received, 1);
        assert_eq!(context.set_source_rgba_received, 1);
        assert_eq!(context.scale_received, 1);
        assert_eq!(context.select_font_face_received, 1);
        assert_eq!(context.set_font_size_received, 1);
        assert_eq!(context.show_text_received, 1);
        assert_eq!(context.stroke_received, 1);
    }
}
