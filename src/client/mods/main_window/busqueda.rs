use crate::client::mods::{
    lib::{call, debug},
    structs::{
        args::{AgregarProductoAVenta, GetProductosFiltrado},
        Nav, Pos, ValuableSH, VentaSH,
    },
};
use serde_wasm_bindgen::from_value;
use std::ops::Deref;
use sycamore::{
    futures::{spawn_local_scoped, spawn_local},
    prelude::*,
    reactive::create_signal,
    Props,
};

#[derive(Props)]
pub struct SearchProps {
    search: Signal<String>,
    nav: Signal<Nav>,
    pos: Signal<Pos>,
    search_aux: Signal<bool>,
}

async fn search(filtro: impl Into<String>) -> Vec<ValuableSH> {
    let res = call(
        "get_productos_filtrado",
        GetProductosFiltrado {
            filtro: filtro.into().as_str(),
        },
    )
    .await;
    from_value::<Vec<ValuableSH>>(res).unwrap()
}

async fn add_to_sale(producto: ValuableSH, pos: bool) -> VentaSH {
    debug(&producto, 38, "busqueda");
    let res = call(
        "agregar_producto_a_venta",
        AgregarProductoAVenta {
            prod: producto,
            pos,
        },
    )
    .await;
    from_value::<VentaSH>(res).unwrap()
}

#[component]
pub fn Busqueda(props: SearchProps) -> View {
    let busqueda: Signal<Vec<ValuableSH>> = create_signal(Vec::new());
    let actual: Signal<Option<(u8, ValuableSH)>> = create_signal(None);
    let actual_selector = create_selector(move || actual.get_clone().map(|o| o.1));
    create_memo(move || {
        props.search.track();
        spawn_local(async move {
            let res = search(props.search.get_clone()).await;
            if res.len() > 0 {
                actual.set(Some((0, res[0].clone())));
                busqueda.set(res);
            } else {
                busqueda.set(Vec::new());
                actual.set(None);
            }
        });
    });

    create_memo(move || {
        props.nav.with(|n| match n {
            Nav::Up => {
                debug(&actual.deref(), 76, "busqueda");
                if let Some((i, _)) = actual.get_clone() {
                    if i > 0 {
                        actual.set(Some((
                            i - 1,
                            busqueda.with(|b| match &b[i as usize - 1] {
                                ValuableSH::Prod((_, p)) => ValuableSH::Prod((1, p.clone())),
                                ValuableSH::Pes((_, p)) => ValuableSH::Pes((0.0, p.clone())),
                                ValuableSH::Rub((_, r)) => ValuableSH::Rub((0, r.clone())),
                            }),
                        )));
                    }
                }
                props.nav.set(Nav::None);
            }
            Nav::Down => {
                debug(&actual.get_clone(), 92, "busqueda");
                if let Some((i, _)) = actual.get_clone() {
                    if i < busqueda.with(|b| b.len() as u8) - 1 {
                        actual.set(Some((
                            i + 1,
                            busqueda.with(|b| match &b[i as usize + 1] {
                                ValuableSH::Prod((_, p)) => ValuableSH::Prod((1, p.clone())),
                                ValuableSH::Pes((_, p)) => ValuableSH::Pes((0.0, p.clone())),
                                ValuableSH::Rub((_, r)) => ValuableSH::Rub((0, r.clone())),
                            }),
                        )));
                    }
                }
                props.nav.set(Nav::None);
            }
            Nav::Enter => {
                if let Some((_, act)) = actual.get_clone() {
                    spawn_local_scoped(async move {
                        // let sale ;
                        let res = add_to_sale(
                            act.clone(),
                            props.pos.with(|p| match p {
                                Pos::A { venta, .. } => {
                                    // sale = venta.clone();
                                    true
                                }
                                Pos::B { venta, .. } => {
                                    // sale = venta.clone();
                                    false
                                }
                            }),
                        )
                        .await;
                        // sale.set(Venta::from_shared(res.clone()));
                        props.search_aux.set(true);
                        props.nav.set(Nav::None);
                    });
                }
            }
            Nav::Esc => {
                if actual.with(|a| a.is_some()) {
                    //actual.set(None);
                }
                props.nav.set(Nav::None);
            }
            Nav::None => (),
        })
    });

    view!(
        section(id="cuadro-principal"){
            (match actual_selector.get_clone(){
                Some(act) => view!(ul(class="no-bullets"){
                    Keyed(
                        list = busqueda,
                        view=move |x|{
                            let mut class="";
                            if act == x{
                                class = "actual";
                            }

                            view!(li(class = class){
                                (match &x{
                                    ValuableSH::Prod(p) => format!("{} {} {} {} {}",p.1.tipo_producto,p.1.marca,p.1.variedad,p.1.presentacion.get_cantidad(),p.1.presentacion.get_string()),
                                    ValuableSH::Pes(p) => p.1.descripcion.as_ref().to_string(),
                                    ValuableSH::Rub(r) => r.1.descripcion.as_ref().to_string(),
                                })
                            })
                        },
                        key=|x|match &x{
                            ValuableSH::Prod(p) => p.1.codigos_de_barras[0],
                            ValuableSH::Pes(p) => p.1.codigo,
                            ValuableSH::Rub(r) => r.1.codigo,
                        }
                    )
                }),
                None => view!(),
            })
        }
    )
}
