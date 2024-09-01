use crate::mods::{
    lib::debug, main_window::{pago::*, resumen_pago::*, *}, structs::{Cliente, Config, Cuenta, MedioPago, Venta}
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
}

#[component]
pub fn Pagos<G: Html>(cx: Scope, props: ResumenProps) -> View<G> {
    let (venta, venta1, venta2) = (
        props.venta.clone(),
        props.venta.clone(),
        props.venta.clone(),
    );
    let conf = props.config.clone();
    let conf1 = props.config.clone();
    let pagos = create_signal(cx, venta.get().pagos.clone());
    let medios = create_rc_signal({
        let filtrado = conf
            .get()
            .medios_pago
            .iter()
            .cloned()
            .filter(|m| m.id != 0)
            .collect::<Vec<MedioPago>>();
        match venta.get().cliente.clone() {
            Cliente::Final => filtrado,
            Cliente::Regular(cli) => match cli.limite {
                Cuenta::Auth(_) => conf.get().medios_pago.clone(),
                Cuenta::Unauth => filtrado,
            },
        }
    });
    let medios2 = medios.clone();
    create_memo(cx, move || {
        venta.track();
        debug(venta.get(),51,"pagos");
        let filtrado = conf
            .get()
            .medios_pago
            .iter()
            .cloned()
            .filter(|m| m.id != 0)
            .collect::<Vec<MedioPago>>();
        medios2.set(match venta.get().cliente.clone() {
            Cliente::Final => filtrado,
            Cliente::Regular(cli) => match cli.limite {
                Cuenta::Auth(_) => conf.get().medios_pago.clone(),
                Cuenta::Unauth => filtrado,
            },
        })
    });
    debug(conf1.get(),64,"pagos");
    debug(medios.clone(),65,"pagos");
    let logg = pagos.get().len();
    let state = create_rc_signal(String::new());
    let s2 = RcSignal::clone(&state);
    let memo = create_memo(cx, move || {
        s2.track();
        debug(s2.get(),71,"pagos");
    });
    //let medios = create_signal(cx, props.config.get().medios_pago.clone());
    debug(logg,74,"pagos");
    view!(cx,
        article(id="pagos"){
            Keyed(
                iterable = pagos,
                view=move |cx,x|{
                    PagoComp(cx,PagoProps::new(true, create_rc_signal(vec![x.medio_pago]), x.monto, None))
                },
                key=|x|x.int_id
            )
            PagoComp(pagado=false, opciones=medios.clone(), monto=venta1.get().monto_total - venta1.get().monto_pagado, state=Some(state))
        }
    )
}
