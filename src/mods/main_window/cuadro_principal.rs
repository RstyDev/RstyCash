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
    let pos = create_signal_from_rc(cx, props.pos);
    let venta_a = create_signal_from_rc(cx, props.venta_a);
    let venta_b = create_signal_from_rc(cx, props.venta_b);
    let config = create_signal_from_rc(cx, props.config);

    view! {cx,
        section(id="cuadro-principal"){
            section(class="ayb"){
                a(id="v-a",class=format!("a-boton {}",match pos.get().as_ref(){true=>"v-actual",false=>""})){
                    "Venta A"
                }
                a(id="v-a",class=format!("a-boton {}",match pos.get().as_ref(){true=>"",false=>"v-actual"})){
                    "Venta B"
                }
            }
            CuadroVenta(venta=match pos.get().as_ref(){
                true => Rc::from(venta_a.get()),
                false => Rc::from(venta_b.get()),
            },config=config.get())
        }
    }
}
