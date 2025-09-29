use wasm_bindgen::prelude::*;

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

    // ---

    let murz = Cat::new();
    murz.set_age(5);
    log(&murz.render());


}
