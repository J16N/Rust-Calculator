mod interpreter;
mod parser;
mod scanner;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-calc!");
}

#[wasm_bindgen]
pub fn calculate(expression: String) -> Result<String, String> {
    let tokens = scanner::tokenize(expression)?;
    let expression = parser::parse(tokens)?;
    todo!();
}
