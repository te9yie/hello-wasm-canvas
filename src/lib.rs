use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

struct App {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    msec: f64,
}

impl App {
    fn new(canvas: HtmlCanvasElement) -> Self {
        let context = canvas.get_context("2d").unwrap();
        let context = context.unwrap();
        let context = context
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context.set_font("100% monospace");
        Self {
            canvas,
            context,
            msec: 0.0,
        }
    }

    fn update(&mut self, delta: f64) {
        self.msec += delta;
    }

    fn draw(&self) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;

        self.context.clear_rect(0.0, 0.0, width, height);

        let text = format!("Hello WASM! {:.2} sec", self.msec / 1000.0);
        self.context.fill_text(&text, 10.0, 20.0).unwrap();
    }
}

fn create_canvas(width: u32, height: u32) -> HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_width(width);
    canvas.set_height(height);
    body.append_child(&canvas).unwrap();
    canvas
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let window = web_sys::window().unwrap();
    let _ = window.request_animation_frame(f.as_ref().unchecked_ref());
}

#[wasm_bindgen(start)]
pub fn main() {
    let canvas = create_canvas(400, 400);
    let mut app = App::new(canvas);

    app.draw();

    {
        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();
        let mut last = performance.now();

        let f = Rc::new(RefCell::new(None));
        let ff = Rc::clone(&f);
        *ff.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let now = performance.now();
            let delta = now - last;
            last = now;
            app.update(delta);
            app.draw();
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        request_animation_frame(ff.borrow().as_ref().unwrap());
    }
}
