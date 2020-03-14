use crate::node::Arc;
use crate::node::Circle;
use crate::node::Curve;
use crate::node::Line;
use crate::node::Polygon;
use crate::node::Rectangle;
use crate::node::Text;
use crate::node::Triangle;

#[derive(Debug, Clone)]
pub enum Layer {
    Rectangle(Rectangle),
    Circle(Circle),
    Arc(Arc),
    Triangle(Triangle),
    Polygon(Polygon),
    Line(Line),
    Curve(Curve),
    Text(Text),
}
