use crate::mods::{
    main_window::productos::*,
    structs::{Config, Venta},
};
use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[derive(Prop)]
pub struct VentaProps {
    pub venta: Rc<Venta>,
    pub config: Rc<Config>,
}
#[component]
pub fn CuadroVenta<G: Html>(cx: Scope, props: VentaProps) -> View<G> {
    let venta = create_signal_from_rc(cx, props.venta);
    let config = create_signal_from_rc(cx, props.config);
    view! {cx,
        section(id="cuadro-venta"){
            Productos(venta=venta.get(),config=config.get())
            section(id="monto-total"){"TOTAL "(venta.get().monto_total)}
        }
    }
}
