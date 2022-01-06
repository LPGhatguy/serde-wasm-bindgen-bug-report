mod ffi;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Foo {
    A { value: u64 },
}

#[wasm_bindgen]
pub fn get_value() -> Result<JsValue, JsValue> {
    Ok(ffi::to_js(&Foo::A { value: 0 })?)
}

#[wasm_bindgen]
pub fn give_value(value: JsValue) -> Result<bool, JsValue> {
    let _value: Foo = ffi::from_js(value)?;
    Ok(true)
}
