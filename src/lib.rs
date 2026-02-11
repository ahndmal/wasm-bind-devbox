use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

#[wasm_bindgen(module = "/app.js")]
extern "C" {
    type Cat;

    #[wasm_bindgen(constructor)]
    fn new() -> Cat;

    #[wasm_bindgen(method, getter)]
    fn age(this: &Cat) -> u32;

    #[wasm_bindgen(method, setter)]
    fn set_age(this: &Cat, number: u32) -> Cat;

    #[wasm_bindgen(method)]
    fn render(this: &Cat) -> String;

    fn hello() -> String;

}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p").unwrap();
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val).unwrap();

    let canvas_el = document.query_selector("canvas#canvas").unwrap().unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas_el
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();

    let img = HtmlImageElement::new().unwrap();
    img.set_src("cat_1.jpg");

    let cat_image_el = document.query_selector("img#cat_1").unwrap().unwrap();

    log(&cat_image_el.node_name().as_str());

    log(hello().as_str());

    // let cat_image = img
    //     .dyn_into::<web_sys::HtmlCanvasElement>()
    //     .map_err(|_| ())
    //     .unwrap();

    match context.draw_image_with_html_image_element(&img, 10.0, 10.0) {
        Ok(x) => {}
        Err(err) => log(err.as_string().unwrap().as_str()),
    };

    // ---

    let murz = Cat::new();
    murz.set_age(5);

    log(&murz.render());
}
