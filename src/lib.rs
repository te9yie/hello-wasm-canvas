use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let context = canvas.get_context("2d").unwrap();
    let context = context
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.set_font("100% sans-serif");
    context.fill_text("Hello WASM!", 10.0, 20.0).unwrap();

    context.set_fill_style(&"red".into());
    context.fill_rect(20.0, 30.0, 40.0, 40.0);

    context.set_fill_style(&"green".into());
    context.fill_rect(40.0, 50.0, 40.0, 40.0);
}
