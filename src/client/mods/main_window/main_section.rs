use crate::client::mods::{
    main_window::{
        busqueda::Busqueda, cuadro_principal::CuadroPrincipal, resumen_pago::ResumenPago,
    },
    structs::{Buscando, Config, Venta},
};
use sycamore::prelude::*;
#[derive(Props, Clone, Debug, PartialEq)]
pub struct SectionProps {
    buscando: Signal<Buscando>,
    venta: Signal<Venta>,
    config: Signal<Config>,
}
#[allow(non_snake_case)]
#[component]
pub fn MainSection(props: SectionProps) -> View {
    let buscando = create_selector(move || props.buscando.get_clone());

    // create_memo(cx, move || {});
    view!(
        (match buscando.get_clone() {
            Buscando::True {
                search,
                nav,
                pos,
                aux,
                other_sale,
                focus,
            } => {
                view!(
                  Busqueda(search = search.clone(), nav = nav.clone(), pos = pos.clone(), search_aux = aux.clone())
                  ResumenPago(venta=props.venta.clone(),pos=pos.clone(),config=props.config.clone(),other_sale=other_sale.clone(), focus=focus.clone())
                )
            }
            Buscando::False {
                pos,
                focus,
                other_sale,
                ..
            } => {
                view!(
                  CuadroPrincipal(pos= pos,focus=focus)
                  ResumenPago(venta=props.venta,pos=pos.clone(),config=props.config.clone(),other_sale=other_sale.clone(), focus=focus.clone())
                )
            }
        })
    )
}
