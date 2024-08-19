use crate::mods::{
    main_window::{cuadro_venta::*, main_page::*},
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

#[component]
pub fn CuadroPrincipal<G: Html>(cx: Scope, props: StateProps) -> View<G> {
    let pos = props.pos.clone();
    let pos2 = props.pos.clone();
    view! {cx,
        section(id="cuadro-principal"){
            section(class="ayb"){
                a(id="v-a",class=format!("a-boton {}",match props.pos.clone().get().as_ref(){true=>"v-actual",false=>""})){
                    "Venta A"
                }
                a(id="v-a",class=format!("a-boton {}",match pos.get().as_ref(){true=>"",false=>"v-actual"})){
                    "Venta B"
                }
            }
            CuadroVenta(venta=match pos2.get().as_ref(){
                true => Rc::from(props.venta_a.get()),
                false => Rc::from(props.venta_b.get()),
            },config=props.config.get())
        }
    }
}
