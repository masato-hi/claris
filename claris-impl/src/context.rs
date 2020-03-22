use cairo::{Context as RealContext, FontSlant, FontWeight, LineCap, Surface};

pub trait Context {
    fn move_to(&mut self, x: f64, y: f64);
    fn save(&mut self);
    fn restore(&mut self);
    fn fill(&mut self);
    fn stroke(&mut self);
    fn set_source_rgba(&mut self, r: f64, g: f64, b: f64, a: f64);
    fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64);
    fn translate(&mut self, x: f64, y: f64);
    fn scale(&mut self, x: f64, y: f64);
    fn set_line_width(&mut self, width: f64);
    fn set_line_cap(&mut self, cap: LineCap);
    fn arc(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64);
    fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
    fn line_to(&mut self, x: f64, y: f64);
    fn close_path(&mut self);
    fn select_font_face(&mut self, family: &str, slant: FontSlant, weight: FontWeight);
    fn set_font_size(&mut self, size: f64);
    fn show_text(&mut self, text: &str);
}

pub struct ContextImpl {
    context: RealContext,
}

impl ContextImpl {
    pub fn new(surface: &Surface) -> Self {
        Self {
            context: RealContext::new(surface),
        }
    }
}

impl Context for ContextImpl {
    fn move_to(&mut self, x: f64, y: f64) {
        self.context.move_to(x, y);
    }

    fn save(&mut self) {
        self.context.save();
    }

    fn restore(&mut self) {
        self.context.restore();
    }

    fn fill(&mut self) {
        self.context.fill();
    }

    fn stroke(&mut self) {
        self.context.stroke();
    }

    fn set_source_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.context.set_source_rgba(r, g, b, a);
    }

    fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.context.rectangle(x, y, width, height);
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.context.translate(x, y);
    }

    fn scale(&mut self, x: f64, y: f64) {
        self.context.scale(x, y);
    }

    fn set_line_width(&mut self, width: f64) {
        self.context.set_line_width(width);
    }

    fn set_line_cap(&mut self, cap: LineCap) {
        self.context.set_line_cap(cap);
    }

    fn arc(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
        self.context.arc(xc, yc, radius, angle1, angle2);
    }

    fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        self.context.curve_to(x1, y1, x2, y2, x3, y3);
    }

    fn line_to(&mut self, x: f64, y: f64) {
        self.context.line_to(x, y);
    }

    fn close_path(&mut self) {
        self.context.close_path();
    }

    fn select_font_face(&mut self, family: &str, slant: FontSlant, weight: FontWeight) {
        self.context.select_font_face(family, slant, weight);
    }

    fn set_font_size(&mut self, size: f64) {
        self.context.set_font_size(size);
    }

    fn show_text(&mut self, text: &str) {
        self.context.show_text(text);
    }
}
