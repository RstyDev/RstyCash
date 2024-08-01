use crate::mods::{
    main_window::pago::*,
    main_window::resumen_pago::*,
    main_window::*,
    structs::{Config, Venta},
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
    let venta = create_signal_from_rc(cx, props.venta);
    let pagos = create_signal(cx, venta.get().pagos.clone());
    let logg = pagos.get().len();
    let state = create_rc_signal(String::new());
    let s2 = RcSignal::clone(&state);
    let memo = create_memo(cx, move || {
        log("aca memo");
        s2.get();
        log(format!("{}", s2.get()).as_str())
    });
    //let medios = create_signal(cx, props.config.get().medios_pago.clone());
    log(&logg.to_string());
    view!(cx,
        article(id="pagos"){
            Keyed(
                iterable = pagos,
                view=|cx,x|{
                    PagoComp(cx,PagoProps::new(true, vec![], 100.1, None))
                },
                key=|x|x.int_id
            )
            PagoComp(pagado=false, opciones=vec![], monto=10.1, state=Some(state))
        }
        // p(){
        //     "a ver como va :"(memo.get())
        // }
    )
}
