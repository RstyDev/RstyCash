use crate::mods::{
    lib::{call, debug},
    structs::{
        args::{AgregarProductoAVenta, GetProductosFiltrado},
        Nav, Pos, ValuableSH, Venta, VentaSHC,
    },
};
use serde_wasm_bindgen::from_value;
use sycamore::{
    flow::Keyed,
    futures::spawn_local_scoped,
    prelude::{
        component, create_memo, create_selector, view, Html, Prop, RcSignal, Scope, Signal, View,
    },
    reactive::create_signal,
};

#[derive(Prop)]
pub struct SearchProps {
    search: RcSignal<String>,
    nav: RcSignal<Nav>,
    pos: RcSignal<Pos>,
    search_aux: RcSignal<bool>,
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

async fn add_to_sale(producto: ValuableSH, pos: bool) -> VentaSHC {
    debug(&producto, 38, "busqueda");
    let res = call(
        "agregar_producto_a_venta",
        AgregarProductoAVenta {
            prod: producto,
            pos,
        },
    )
    .await;
    from_value::<VentaSHC>(res).unwrap()
}

#[component]
pub fn Busqueda<G: Html>(cx: Scope, props: SearchProps) -> View<G> {
    let filtro = props.search.clone();
    let busqueda: &Signal<Vec<ValuableSH>> = create_signal(cx, Vec::new());
    let actual: &Signal<Option<(u8, ValuableSH)>> = create_signal(cx, None);
    let actual_selector = create_selector(cx, || actual.get().as_ref().clone().map(|o| o.1));
    let nav = props.nav.clone();
    create_memo(cx, move || {
        filtro.track();
        let filtro = filtro.clone();
        spawn_local_scoped(cx, async move {
            let res = search(filtro.get().as_ref()).await;
            if res.len() > 0 {
                actual.set(Some((0, res[0].clone())));
                busqueda.set(res);
            } else {
                busqueda.set(Vec::new());
                actual.set(None);
            }
        });
    });

    create_memo(cx, move || {
        match nav.get().as_ref() {
            Nav::Up => {
                debug(actual.get().as_ref(), 76, "busqueda");
                if let Some((i, _)) = actual.get().as_ref() {
                    if *i > 0 {
                        actual.set(Some((
                            i - 1,
                            match &busqueda.get().as_ref()[*i as usize - 1] {
                                ValuableSH::Prod((_, p)) => ValuableSH::Prod((1, p.clone())),
                                ValuableSH::Pes((_, p)) => ValuableSH::Pes((0.0, p.clone())),
                                ValuableSH::Rub((_, r)) => ValuableSH::Rub((0, r.clone())),
                            },
                        )));
                    }
                }
                nav.set(Nav::None);
            }
            Nav::Down => {
                debug(actual.get().as_ref(), 92, "busqueda");
                if let Some((i, _)) = actual.get().as_ref() {
                    if *i < busqueda.get().as_ref().len() as u8 - 1 {
                        actual.set(Some((
                            i + 1,
                            match &busqueda.get().as_ref()[*i as usize + 1] {
                                ValuableSH::Prod((_, p)) => ValuableSH::Prod((1, p.clone())),
                                ValuableSH::Pes((_, p)) => ValuableSH::Pes((0.0, p.clone())),
                                ValuableSH::Rub((_, r)) => ValuableSH::Rub((0, r.clone())),
                            },
                        )));
                    }
                }
                nav.set(Nav::None);
            }
            Nav::Enter => {
                if let Some((_, act)) = actual.get().as_ref().clone() {
                    let pos = props.pos.clone();
                    let aux = props.search_aux.clone();
                    let nav = nav.clone();
                    spawn_local_scoped(cx, async move {
                        let sale;
                        let res = add_to_sale(
                            act,
                            match pos.get().as_ref() {
                                Pos::A { venta, .. } => {
                                    sale = venta.clone();
                                    true
                                }
                                Pos::B { venta, .. } => {
                                    sale = venta.clone();
                                    false
                                }
                            },
                        )
                        .await;
                        sale.set(Venta::from_shared_complete(res.clone()));
                        aux.set(true);
                        nav.set(Nav::None);
                    });
                }
            }
            Nav::Esc => {
                if actual.get().as_ref().is_some() {
                    //actual.set(None);
                }
                nav.set(Nav::None);
            }
            Nav::None => (),
        }
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
                                    ValuableSH::Prod(p) => format!("{} {} {} {} {}",p.1.tipo_producto,p.1.marca,p.1.variedad,p.1.presentacion.get_cantidad(),p.1.presentacion.get_string()),
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
        }
    )
}
