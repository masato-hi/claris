use cairo::Context;
use log::debug;

use self::super::node::Layer;

mod arc;
mod circle;
mod curve;
mod line;
mod polygon;
mod rectangle;
mod text;
mod triangle;

use arc::Arc as ArcRenderer;
use circle::Circle as CircleRenderer;
use curve::Curve as CurveRenderer;
use line::Line as LineRenderer;
use polygon::Polygon as PolygonRenderer;
use rectangle::Rectangle as RectangleRenderer;
use text::Text as TextRenderer;
use triangle::Triangle as TriangleRenderer;

pub fn render(context: &mut Context, layer: Layer) {
  context.save();
  debug!("{:?}", layer);
  match layer {
    Layer::Rectangle(x) => {
      RectangleRenderer::render(context, x);
    }
    Layer::Circle(x) => {
      CircleRenderer::render(context, x);
    }
    Layer::Arc(x) => {
      ArcRenderer::render(context, x);
    }
    Layer::Triangle(x) => {
      TriangleRenderer::render(context, x);
    }
    Layer::Polygon(x) => {
      PolygonRenderer::render(context, x);
    }
    Layer::Line(x) => {
      LineRenderer::render(context, x);
    }
    Layer::Curve(x) => {
      CurveRenderer::render(context, x);
    }
    Layer::Text(x) => {
      TextRenderer::render(context, x);
    }
  };
  context.restore();
}
