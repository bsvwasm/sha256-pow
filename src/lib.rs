extern crate console_error_panic_hook;

pub mod proof_of_work;
pub use proof_of_work::*;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};


#[wasm_bindgen]
pub fn xor(a: &[u8], b: &[u8]) -> Result<Vec<u8>, JsValue> {
    if a.len() != b.len() {
        return Err(JsValue::from_str("A and B lengths do not match!"));
    }

    let xor_array = b.iter()
        .zip(a.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();

    Ok(xor_array)
}