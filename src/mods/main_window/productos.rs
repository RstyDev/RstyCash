use crate::mods::{
    main_window::{
        pagos::Pagos, producto::Prod, resumen_pago::ResumenPago, resumen_pago::ResumenProps,
    },
    structs::{
        Cliente, Config, Formato, Mayusculas, MedioPago, Pago, Pesable, Presentacion, Producto,
        Proveedor, RelacionProdProv, Rubro, Valuable, Venta, Windows,
    },
    Login,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::{rc::Rc, sync::Arc};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::rt::Event;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
#[derive(Prop)]
pub struct ProdsProps {
    venta: Rc<Venta>,
    config: Rc<Config>,
}
#[component]
pub fn Productos<G: Html>(cx: Scope, props: ProdsProps) -> View<G> {
    let prods = create_signal(cx, props.venta.productos.clone());
    let conf = create_signal_from_rc(cx, props.config);

    view! {cx,
        section(id="productos"){
            article(class="articulo"){
                section(class="descripcion"){
                    p{"DESCRIPCION"}
                }
                section(class="cantidad"){
                    p{"CANTIDAD"}
                }
                section(class="monto"){
                    p{"UNIDAD"}
                }
                section(){
                    p{"TOTAL PARCIAL"}
                }
            }
            Keyed(
                iterable = prods,
                view = move |cx,x|{
                    view!{cx,Prod(valuable = x, conf = conf.get())}
                },
                key = |x|{x.id()}
            )
        }
    }
}
