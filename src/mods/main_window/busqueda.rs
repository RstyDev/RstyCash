use crate::mods::{lib::debug, structs::{Nav, Pos, ValuableSH, Venta, VentaSHC}};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use sycamore::{
    flow::Keyed,
    futures::spawn_local_scoped,
    prelude::{
        component, create_effect, create_memo, create_selector, view, Html, Prop, RcSignal, Scope, Signal, View
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
#[derive(Serialize,Deserialize)]
struct ProdArg{
    prod: ValuableSH,
    pos: bool,
}

#[derive(Prop)]
pub struct SearchProps {
    search: RcSignal<String>,
    nav: RcSignal<Nav>,
    pos: RcSignal<Pos>,
}

async fn search(filtro: impl Into<String>)->Vec<ValuableSH>{
    let filtro = to_value(&SearchArg {
        filtro: filtro.into().as_str(),
    })
    .unwrap();
    let res = invoke("get_productos_filtrado", filtro).await;
    from_value::<Vec<ValuableSH>>(res).unwrap()
}

async fn add_to_sale(producto: ValuableSH, pos: bool)->VentaSHC{
    let value = to_value(&ProdArg{prod: producto, pos}).unwrap();
    let res = invoke("agregar_producto_a_venta", value).await;
    from_value::<VentaSHC>(res).unwrap()
}

#[component]
pub fn Busqueda<G: Html>(cx: Scope, props: SearchProps) -> View<G> {
    let filtro = props.search.clone();
    let busqueda: &Signal<Vec<ValuableSH>> = create_signal(cx, Vec::new());
    let actual: &Signal<Option<(u8,ValuableSH)>> = create_signal(cx, None);
    let actual_selector = create_selector(cx, ||actual.get().as_ref().clone().map(|o|o.1));
    let nav = props.nav.clone();
    create_memo(cx, move || {
        filtro.track();
        debug(filtro.clone(),52,"busqueda");
        let filtro = filtro.clone();
        spawn_local_scoped(cx, async move {
            let res = search(filtro.get().as_ref()).await;
            if res.len()>0{
                actual.set(Some((0,res[0].clone())));
                busqueda.set(res);
            }else{
                busqueda.set(Vec::new());
                actual.set(None);
            }
        });
    });
    create_memo(cx, move ||{
        match nav.get().as_ref(){
            Nav::Up => {
                debug("UPUP",65,"busqueda");
                if let Some((i,_))=actual.get().as_ref(){
                    if *i > 0{
                        actual.set(Some((i-1,busqueda.get().as_ref()[*i as usize-1].clone())));
                    }
                }
            },
            Nav::Down => {
                debug("DWN",73,"busqueda");
                if let Some((i,_))=actual.get().as_ref(){
                    if *i < busqueda.get().as_ref().len() as u8 -1{
                        actual.set(Some((i+1,busqueda.get().as_ref()[*i as usize + 1].clone())));
                    }
                }
            },
            Nav::Enter => {
                debug("ENT",81,"busqueda");
                if let Some((_,act)) = actual.get().as_ref().clone(){
                    let pos = props.pos.clone();
                    
                    spawn_local_scoped(cx, async move {
                        let sale;
                        let res=add_to_sale(act, match pos.get().as_ref(){
                            Pos::A { venta, config:_, clientes:_ } => {
                                sale=venta.clone();
                                true
                            },
                            Pos::B { venta, config:_, clientes:_ } => {
                                sale=venta.clone();
                                false
                            },
                        }).await;
                        debug(res.clone(),110,"busqueda");
                        sale.set(Venta::from_shared_complete(res));
                    });
                }
            },
            Nav::Esc => {
                debug("ESC",87,"busqueda");
                if actual.get().as_ref().is_some(){
                    //actual.set(None);
                }
            },
        }
    });
    
    // create_effect(cx, ||{
    //     if busqueda.get().as_ref().len() == 0{
    //         actual.set(None);
    //     }else {
    //         actual.set(Some((0,busqueda.get().as_ref()[0].clone())));
    //     }
    //     debug(busqueda.get().as_ref().iter().map(|v|match v{
    //         ValuableSH::Prod(p) => format!("{} {} {}",p.1.tipo_producto,p.1.marca,p.1.variedad),
    //         ValuableSH::Pes(p) => p.1.descripcion.as_ref().to_string(),
    //         ValuableSH::Rub(r) => r.1.descripcion.as_ref().to_string(),
    //     }).collect::<Vec<String>>(),61,"busqueda")
        
    // });
    create_effect(cx, ||{
        debug(actual.get(),81,"busqueda");
    });

    view!(cx,
        section(id="cuadro-principal"){
            (match actual_selector.get().as_ref().clone(){
                Some(act) => view!(cx,ul(class="no-bullets"){
                    Keyed(
                        iterable = busqueda,
                        view=move |cx,x|{
                            let mut class="";
                            if act == x{
                                class = "actual";
                            }
                            
                            view!(cx,li(class = class){
                                (match &x{
                                    ValuableSH::Prod(p) => format!("{} {} {}",p.1.tipo_producto,p.1.marca,p.1.variedad),
                                    ValuableSH::Pes(p) => p.1.descripcion.as_ref().to_string(),
                                    ValuableSH::Rub(r) => r.1.descripcion.as_ref().to_string(),
                                })
                            })
                        },
                        key=|x|match &x{
                            ValuableSH::Prod(p) => p.1.codigo_de_barras,
                            ValuableSH::Pes(p) => p.1.codigo,
                            ValuableSH::Rub(r) => r.1.codigo,
                        }
                    )
                }),
                None => view!(cx,div()),
            

            })
            // ul(class="no-bullets"){
                
            //     Keyed(
            //         iterable = busqueda,
            //         view=move |cx,x|{
            //             let mut class="";
            //             if let Some(act) = actual.get().as_ref().clone(){
            //                 if act == x{
            //                     class = "actual";
            //                 }
            //             }
            //             view!(cx,li(class = class){
            //                 (match &x{
            //                     ValuableSH::Prod(p) => format!("{} {} {}",p.1.tipo_producto,p.1.marca,p.1.variedad),
            //                     ValuableSH::Pes(p) => p.1.descripcion.as_ref().to_string(),
            //                     ValuableSH::Rub(r) => r.1.descripcion.as_ref().to_string(),
            //                 })
            //             })
            //         },
            //         key=|x|match x{
            //             ValuableSH::Prod(p) => p.1.codigo_de_barras,
            //             ValuableSH::Pes(p) => p.1.codigo,
            //             ValuableSH::Rub(r) => r.1.codigo,
            //         }
            //     )
            // }
        }
    )
}
