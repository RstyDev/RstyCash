use sycamore::prelude::*;

use crate::mods::{
    main_window::{
        busqueda::Busqueda, cuadro_principal::CuadroPrincipal, resumen_pago::ResumenPago,
    },
    structs::{Buscando, Config, Venta},
};
#[derive(Prop, Clone, Debug, PartialEq)]
pub struct SectionProps {
    buscando: RcSignal<Buscando>,
    venta: RcSignal<Venta>,
    config: RcSignal<Config>,
}

#[component]
pub fn MainSection<G: Html>(cx: Scope, props: SectionProps) -> View<G> {
    let buscando_aux = props.buscando.clone();
    let buscando = create_selector(cx, move || buscando_aux.get().as_ref().clone());
    let venta = props.venta.clone();
    let venta1 = props.venta.clone();
    let config = props.config.clone();
    create_memo(cx, move || {});
    view!(
        cx,
        (match buscando.get().as_ref().clone() {
            Buscando::True {
                search,
                nav,
                pos,
                aux,
                other_sale,
            } => {
                let venta = venta.clone();
                let config = config.clone();
                let pos1 = pos.clone();
                view!(cx,
                  Busqueda(search = search.clone(), nav = nav.clone(), pos = pos.clone(), search_aux = aux.clone())
                  ResumenPago(venta=venta.clone(),pos=pos1.clone(),config=config.clone(),other_sale=other_sale.clone())
                )
            }
            Buscando::False {
                pos,
                focus,
                other_sale,
                ..
            } => {
                let config = config.clone();
                let venta = venta1.clone();
                let pos1 = pos.clone();
                view!(cx,
                  CuadroPrincipal(pos= pos.clone())
                  ResumenPago(venta=venta.clone(),pos=pos1.clone(),config=config.clone(),other_sale=other_sale.clone())
                )
            }
        })
    )
}
