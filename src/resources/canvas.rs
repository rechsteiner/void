use crate::components::point::Point;
use crate::components::shape::ColorRGBA;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};

pub struct Path {
    points: Vec<Point>,
}

impl Path {
    pub fn new() -> Self {
        Path { points: vec![] }
    }

    pub fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        self.points.push(Point { x, y });
        self
    }
}

// Exposes method on CanvasRenderingContext2d, so that we can use the
// "interior mutability" pattern. This allows us to borrow the
// resource immutably, but still mutate to the underlying context.
// TODO: Use RefCell on world.get_resource method instead?
pub struct Canvas {
    context: Rc<RefCell<web_sys::CanvasRenderingContext2d>>,
    element: web_sys::HtmlCanvasElement,
}

impl Canvas {
    pub fn new() -> Canvas {
        let window = web_sys::window().expect("no global `window` exists");
        let document: web_sys::Document =
            window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("canvas").unwrap();
        let element: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context: web_sys::CanvasRenderingContext2d = element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Canvas {
            context: Rc::new(RefCell::new(context)),
            element,
        }
    }

    pub fn width(&self) -> f64 {
        self.element.width() as f64
    }

    pub fn height(&self) -> f64 {
        self.element.height() as f64
    }

    pub fn draw_rectangle(&self, x: f32, y: f32, width: f32, height: f32, color: &ColorRGBA) {
        let context = self.context.borrow_mut();
        context.set_fill_style(&JsValue::from(format!("{}", color)));
        context.fill_rect(x as f64, y as f64, width as f64, height as f64);
    }

    pub fn draw_text(&self, x: f32, y: f32, font: &str, text: &str, color: &ColorRGBA) {
        let context = self.context.borrow_mut();
        context.set_font(font);
        context.set_stroke_style(&JsValue::from(format!("{}", color)));
        context.fill_text(text, x as f64, y as f64).unwrap();
    }

    pub fn draw_path(&self, path: Path, width: f32, color: &ColorRGBA) {
        let context = self.context.borrow_mut();
        context.begin_path();
        context.set_line_width(width as f64);
        context.set_stroke_style(&JsValue::from(format!("{}", color)));
        context.set_fill_style(&JsValue::null());

        for point in path.points {
            context.line_to(point.x as f64, point.y as f64);
        }

        context.stroke();
        context.close_path();
    }

    pub fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        let context = self.context.borrow_mut();
        context.clear_rect(x, y, w, h);
    }

    pub fn translate(&self, x: f64, y: f64) {
        let context = self.context.borrow_mut();
        context.translate(x, y).unwrap();
    }

    pub fn rotate(&self, angle: f64) {
        let context = self.context.borrow_mut();
        context.rotate(angle).unwrap();
    }

    pub fn reset_transform(&self) {
        let context = self.context.borrow_mut();
        context.reset_transform().unwrap();
    }
}
