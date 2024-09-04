use crate::mods::{
    main_window::{cuadro_venta::*, main_page::*},
    structs::{Cliente, Config, Pos, Venta},
};
use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
#[derive(Prop)]
pub struct PrincProps {
    pub venta: RcSignal<Venta>,
    pub config: RcSignal<Config>,
    pub clientes: RcSignal<Vec<Cliente>>,
    pub pos: RcSignal<Pos>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn CuadroPrincipal<G: Html>(cx: Scope, props: PrincProps) -> View<G> {
    view! {cx,
        section(id="cuadro-principal"){
            CuadroVenta(venta=Rc::from(props.venta.get()),config=props.config.get())
        }
    }
}
