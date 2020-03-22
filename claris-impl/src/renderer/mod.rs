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

use crate::Context;
use arc::Arc as ArcRenderer;
use circle::Circle as CircleRenderer;
use curve::Curve as CurveRenderer;
use line::Line as LineRenderer;
use polygon::Polygon as PolygonRenderer;
use rectangle::Rectangle as RectangleRenderer;
use text::Text as TextRenderer;
use triangle::Triangle as TriangleRenderer;

pub fn render(context: &mut dyn Context, layer: Layer) {
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

#[cfg(test)]
mod tests {
    use super::render;
    use crate::node::Arc;
    use crate::node::Circle;
    use crate::node::Curve;
    use crate::node::Layer;
    use crate::node::Line;
    use crate::node::Polygon;
    use crate::node::Rectangle;
    use crate::node::Text;
    use crate::node::Triangle;
    use crate::testing_helpers::stub::ContextImpl;

    #[test]
    fn arc() {
        let mut context = ContextImpl::new();
        let node = Arc::default();
        let layer = Layer::Arc(node);
        render(&mut context, layer);
    }

    #[test]
    fn circle() {
        let mut context = ContextImpl::new();
        let node = Circle::default();
        let layer = Layer::Circle(node);
        render(&mut context, layer);
    }

    #[test]
    fn curve() {
        let mut context = ContextImpl::new();
        let node = Curve::default();
        let layer = Layer::Curve(node);
        render(&mut context, layer);
    }

    #[test]
    fn line() {
        let mut context = ContextImpl::new();
        let node = Line::default();
        let layer = Layer::Line(node);
        render(&mut context, layer);
    }

    #[test]
    fn polygon() {
        let mut context = ContextImpl::new();
        let node = Polygon::default();
        let layer = Layer::Polygon(node);
        render(&mut context, layer);
    }

    #[test]
    fn rectangle() {
        let mut context = ContextImpl::new();
        let node = Rectangle::default();
        let layer = Layer::Rectangle(node);
        render(&mut context, layer);
    }

    #[test]
    fn text() {
        let mut context = ContextImpl::new();
        let node = Text::default();
        let layer = Layer::Text(node);
        render(&mut context, layer);
    }

    #[test]
    fn triangle() {
        let mut context = ContextImpl::new();
        let node = Triangle::default();
        let layer = Layer::Triangle(node);
        render(&mut context, layer);
    }
}
