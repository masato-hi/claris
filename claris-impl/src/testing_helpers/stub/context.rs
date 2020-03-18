use crate::Context;
use cairo::{FontSlant, FontWeight, LineCap};

#[derive(Default)]
pub struct ContextImpl {
    pub move_to_received: i32,
    pub save_received: i32,
    pub restore_received: i32,
    pub fill_received: i32,
    pub stroke_received: i32,
    pub set_source_rgba_received: i32,
    pub rectangle_received: i32,
    pub translate_received: i32,
    pub scale_received: i32,
    pub set_line_width_received: i32,
    pub set_line_cap_received: i32,
    pub arc_received: i32,
    pub curve_to_received: i32,
    pub line_to_received: i32,
    pub close_path_received: i32,
    pub select_font_face_received: i32,
    pub set_font_size_received: i32,
    pub show_text_received: i32,
}

impl ContextImpl {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Context for ContextImpl {
    fn move_to(&mut self, _x: f64, _y: f64) {
        self.move_to_received += 1;
    }

    fn save(&mut self) {
        self.save_received += 1;
    }

    fn restore(&mut self) {
        self.restore_received += 1;
    }

    fn fill(&mut self) {
        self.fill_received += 1;
    }

    fn stroke(&mut self) {
        self.stroke_received += 1;
    }

    fn set_source_rgba(&mut self, _r: f64, _g: f64, _b: f64, _a: f64) {
        self.set_source_rgba_received += 1;
    }

    fn rectangle(&mut self, _x: f64, _y: f64, _width: f64, _height: f64) {
        self.rectangle_received += 1;
    }

    fn translate(&mut self, _x: f64, _y: f64) {
        self.translate_received += 1;
    }

    fn scale(&mut self, _x: f64, _y: f64) {
        self.scale_received += 1;
    }

    fn set_line_width(&mut self, _width: f64) {
        self.set_line_width_received += 1;
    }

    fn set_line_cap(&mut self, _cap: LineCap) {
        self.set_line_cap_received += 1;
    }

    fn arc(&mut self, _xc: f64, _yc: f64, _radius: f64, _angle1: f64, _angle2: f64) {
        self.arc_received += 1;
    }

    fn curve_to(&mut self, _x1: f64, _y1: f64, _x2: f64, _y2: f64, _x3: f64, _y3: f64) {
        self.curve_to_received += 1;
    }

    fn line_to(&mut self, _x: f64, _y: f64) {
        self.line_to_received += 1;
    }

    fn close_path(&mut self) {
        self.close_path_received += 1;
    }

    fn select_font_face(&mut self, _family: &str, _slant: FontSlant, _weight: FontWeight) {
        self.select_font_face_received += 1;
    }

    fn set_font_size(&mut self, _size: f64) {
        self.set_font_size_received += 1;
    }

    fn show_text(&mut self, _text: &str) {
        self.show_text_received += 1;
    }
}
