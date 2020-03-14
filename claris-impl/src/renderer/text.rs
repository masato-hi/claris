use self::super::super::node::Text as Node;
use cairo::Context;

pub struct Text {}

impl Text {
  pub fn render(context: &mut Context, node: Node) {
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
