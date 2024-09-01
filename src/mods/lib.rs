use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
pub fn debug(s: impl std::fmt::Debug, line: u16, file: &str){
    log(format!("*** Linea: {line} *** \n*** File: {file}*** \n{:#?}",s).as_str())
}