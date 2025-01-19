use serde_wasm_bindgen::to_value;
use sycamore::prelude::console_log;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn call(cmd: &str, args: impl serde::ser::Serialize + Sized) -> JsValue {
    let value = to_value(&args).unwrap();
    invoke(cmd, value).await
}
pub fn debug(s: &impl std::fmt::Debug, line: u16, file: &str) {
    console_log!("*** Linea: {line} *** \n*** File: {file}*** \n{:#?}", s)
}

pub fn round(number: i64) -> String {
    format!("{:.2}", number)
}
