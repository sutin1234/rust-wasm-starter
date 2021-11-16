use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

// Return Value
#[wasm_bindgen]
pub fn get_name(name: &str) -> String {
    format!("myName {}", name)
}

#[wasm_bindgen]
pub fn greet2(a: &str) {
    log_many("Hello, {}!", a)
}
