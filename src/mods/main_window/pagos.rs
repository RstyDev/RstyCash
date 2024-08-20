use crate::mods::{
    main_window::{pago::*, resumen_pago::*, *},
    structs::{Config, MedioPago, Venta},
};
use sycamore::{
    prelude::*,
    reactive::{create_memo, create_rc_signal, RcSignal},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use super::resumen_pago::ResumenProps;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[component]
pub fn Pagos<G: Html>(cx: Scope, props: ResumenProps) -> View<G> {
    let venta = props.venta.clone();
    let conf = props.config.clone();
    let pagos = create_signal(cx, venta.get().pagos.clone());
    let medios = create_signal(cx, {
        match venta.get().cliente {
            crate::mods::structs::Cliente::Final => conf
                .get()
                .medios_pago
                .iter()
                .cloned()
                .filter(|m| m.id != 0)
                .collect::<Vec<MedioPago>>(),
            crate::mods::structs::Cliente::Regular(_) => conf.get().medios_pago.clone(),
        }
    });

    let logg = pagos.get().len();
    let state = create_rc_signal(String::new());
    let s2 = RcSignal::clone(&state);
    let memo = create_memo(cx, move || {
        s2.get();
        log(format!("{}", s2.get()).as_str())
    });
    //let medios = create_signal(cx, props.config.get().medios_pago.clone());
    log(&logg.to_string());
    view!(cx,
        article(id="pagos"){
            Keyed(
                iterable = pagos,
                view=move |cx,x|{
                    PagoComp(cx,PagoProps::new(true, vec![x.medio_pago], x.monto, None))
                },
                key=|x|x.int_id
            )
            PagoComp(pagado=false, opciones=medios.get().as_ref().clone(), monto=venta.get().monto_total - venta.get().monto_pagado, state=Some(state))
        }
    )
}
