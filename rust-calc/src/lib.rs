mod interpreter;
mod parser;
mod scanner;
mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn execute(expression: String) -> Result<JsValue, JsError> {
    set_panic_hook();
    let tokens = scanner::tokenize(expression)?;
    let expression = parser::parse(tokens)?;
    let result = interpreter::interpret(expression)?;
    Ok(JsValue::from_str(&result))
}
