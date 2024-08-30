use std::sync::Arc;

use crate::mods::{lib::debug, structs::{Config, Valuable, ValuableSH, Venta}};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use sycamore::{
    flow::Keyed,
    futures::{create_resource, spawn_local_scoped},
    prelude::{
        component, create_effect, create_memo, view, Html, Prop, RcSignal, Scope, Signal, View
    },
    reactive::create_signal,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


#[derive(Serialize, Deserialize)]
struct SearchArg<'a> {
    filtro: &'a str,
}

#[derive(Prop)]
pub struct SearchProps {
    search: RcSignal<String>,
    venta: RcSignal<Venta>,
}

async fn search(filtro: impl Into<String>)->Vec<ValuableSH>{
    let filtro = to_value(&SearchArg {
        filtro: filtro.into().as_str(),
    })
    .unwrap();
    let res = invoke("get_productos_filtrado", filtro).await;
    from_value::<Vec<ValuableSH>>(res).unwrap()
}

#[component]
pub fn Busqueda<G: Html>(cx: Scope, props: SearchProps) -> View<G> {
    let filtro = props.search.clone();
    let busqueda: &Signal<Vec<ValuableSH>> = create_signal(cx, Vec::new());
    create_memo(cx, move || {
        filtro.track();
        debug(filtro.clone(),52);
        let filtro = filtro.clone();
        spawn_local_scoped(cx, async move {
            busqueda.set(search(filtro.get().as_ref()).await)
        });
    });

    
    create_effect(cx, ||{
        debug(busqueda.get().as_ref().iter().map(|v|match v{
            ValuableSH::Prod(p) => format!("{} {} {}",p.1.tipo_producto,p.1.marca,p.1.variedad),
            ValuableSH::Pes(p) => p.1.descripcion.as_ref().to_string(),
            ValuableSH::Rub(r) => r.1.descripcion.as_ref().to_string(),
        }).collect::<Vec<String>>(),61)
    });

    view!(cx,
        section(id="cuadro-principal"){
            Keyed(
                iterable = busqueda,
                view=move |cx,x|{
                    view!(cx,li(){
                        (match &x{
                            ValuableSH::Prod(p) => format!("{} {} {}",p.1.tipo_producto,p.1.marca,p.1.variedad),
                            ValuableSH::Pes(p) => p.1.descripcion.as_ref().to_string(),
                            ValuableSH::Rub(r) => r.1.descripcion.as_ref().to_string(),
                        })
                    })
                },
                key=|x|match x{
                    ValuableSH::Prod(p) => p.1.codigo_de_barras,
                    ValuableSH::Pes(p) => p.1.codigo,
                    ValuableSH::Rub(r) => r.1.codigo,
                }
            )
        }
    )
}
