use js_sys::wasm_bindgen;
use std::sync::Arc;
use sycamore::rt::Event;
use sycamore::{
    prelude::{component, create_signal, view, Html, Keyed, Prop, Scope, View},
    reactive::{RcSignal, Signal},
};
use wasm_bindgen::prelude::*;

use crate::mods::structs::MedioPago;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Prop)]
pub struct PagoProps {
    pagado: bool,
    opciones: Vec<MedioPago>,
    monto: f32,
    state: Option<RcSignal<String>>,
}
impl PagoProps {
    pub fn new(
        pagado: bool,
        opciones: Vec<MedioPago>,
        monto: f32,
        state: Option<RcSignal<String>>,
    ) -> PagoProps {
        PagoProps {
            pagado,
            opciones,
            monto,
            state,
        }
    }
}

#[component]
pub fn PagoComp<G: Html>(cx: Scope, props: PagoProps) -> View<G> {
    let opts = create_signal(cx, props.opciones);

    view! {cx,
        form(id="form-pago"){
            input(type="number",placeholder=props.monto,class="input-monto",disabled = props.pagado)
            select(class="opciones-pagos",disabled = props.pagado){
            Keyed(
                iterable = opts,
                view=|cx,x|view!{cx,
                    option(){(x.medio)}
                },
                key=|x|x.id,
            )
        }
            input(type="submit", value=match props.pagado{true => "Borrar",false => "Pagar"}, on:click=move |a:Event|{
                log("aca");
                a.prevent_default();
                match &props.state{
                   Some(s) => s.set(String::from("Desde Pago")),
                   None => (),
                }
            })
        }
    }
}
