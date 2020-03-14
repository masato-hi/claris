use self::super::Arc;
use self::super::Circle;
use self::super::Curve;
use self::super::Line;
use self::super::Polygon;
use self::super::Rectangle;
use self::super::Text;
use self::super::Triangle;

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
