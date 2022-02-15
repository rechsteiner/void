use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};

// Exposes method on CanvasRenderingContext2d, so that we can use the
// "interior mutability" pattern. This allows us to borrow the
// resource immutably, but still mutate to the underlying context.
// TODO: Use RefCell on world.get_resource method instead?
pub struct Canvas {
    pub context: Rc<RefCell<web_sys::CanvasRenderingContext2d>>,
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

    pub fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        let context = self.context.borrow_mut();
        context.clear_rect(x, y, w, h);
    }

    pub fn set_line_width(&self, value: f64) {
        let context = self.context.borrow_mut();
        context.set_line_width(value);
    }

    pub fn set_font(&self, value: &str) {
        let context = self.context.borrow_mut();
        context.set_font(value);
    }

    pub fn set_fill_style(&self, value: &JsValue) {
        let context = self.context.borrow_mut();
        context.set_fill_style(value);
    }

    pub fn fill_text(&self, text: &str, x: f64, y: f64) {
        let context = self.context.borrow_mut();
        context.fill_text(text, x, y).unwrap();
    }

    pub fn translate(&self, x: f64, y: f64) {
        let context = self.context.borrow_mut();
        context.translate(x, y).unwrap();
    }

    pub fn rotate(&self, angle: f64) {
        let context = self.context.borrow_mut();
        context.rotate(angle).unwrap();
    }

    pub fn set_stroke_style(&self, value: &JsValue) {
        let context = self.context.borrow_mut();
        context.set_stroke_style(value);
    }

    pub fn begin_path(&self) {
        let context = self.context.borrow_mut();
        context.begin_path();
    }

    pub fn close_path(&self) {
        let context = self.context.borrow_mut();
        context.close_path();
    }

    pub fn stroke(&self) {
        let context = self.context.borrow_mut();
        context.stroke();
    }

    pub fn line_to(&self, x: f64, y: f64) {
        let context = self.context.borrow_mut();
        context.line_to(x, y);
    }

    pub fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        let context = self.context.borrow_mut();
        context.stroke_rect(x, y, w, h);
    }

    pub fn reset_transform(&self) {
        let context = self.context.borrow_mut();
        context.reset_transform().unwrap();
    }
}
