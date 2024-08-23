use std::rc::Rc;

use crate::mods::structs::{Config, Valuable, ValuableSH, Venta};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use sycamore::{
    flow::Keyed,
    futures::create_resource,
    prelude::{
        component, create_rc_signal, create_selector, view, Html, Prop, RcSignal, Scope, Signal,
        View,
    },
    reactive::{create_signal, create_signal_from_rc, ReadSignal},
    web::html::view,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct SearchArg<'a> {
    filtro: &'a str,
}

#[derive(Prop)]
pub struct SearchProps {
    search: String,
    venta: RcSignal<Venta>,
}

async fn search(filtro: String)->Vec<ValuableSH>{
    let filtro = to_value(&SearchArg {
        filtro: filtro.as_str(),
    })
    .unwrap();
    let res = invoke("get_productos_filtrado", filtro).await;
    from_value::<Vec<ValuableSH>>(res).unwrap()
}

#[component]
pub fn Busqueda<G: Html>(cx: Scope, props: SearchProps) -> View<G> {
    let filtro = props.search.clone();
    let busqueda = create_resource(cx, async move {
        search(filtro).await
    });
    

    view!(cx,
        section(id="cuadro-principal"){
            (match busqueda.get().as_ref(){
                Some(vals) => {
                    let sig=create_signal(cx,vals.clone());
                    view!(cx,ul(){
                        Keyed(
                            iterable = sig,
                            view=move |cx,x|{
                                view!(cx,li(){
                                    (format!("{}",match &x{
                                        ValuableSH::Prod(p) => p.1.marca.clone(),
                                        ValuableSH::Pes(p) => p.1.descripcion.clone(),
                                        ValuableSH::Rub(r) => r.1.descripcion.clone(),
                                    }))
                                })
                            },
                            key=|x|match x{
                                ValuableSH::Prod(p) => p.1.codigo_de_barras,
                                ValuableSH::Pes(p) => p.1.codigo,
                                ValuableSH::Rub(r) => r.1.codigo,
                            }
                        )
                    })
                },
                None => view!(cx,ul(){}),
            })
        }
    )
}
