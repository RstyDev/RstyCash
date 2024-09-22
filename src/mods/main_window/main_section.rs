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
    let aux1 = props.buscando.clone();
    let buscando = create_selector(cx, move || buscando_aux.get().as_ref().clone());
    let venta = props.venta.clone();
    let config = props.config.clone();
    create_memo(cx, move || {});
    view!(cx,
      (match buscando.get().as_ref().clone(){
        Buscando::True{search,nav,pos, aux } => view!(cx,
          Busqueda(search = search.clone(), nav = nav.clone(), pos = pos.clone(), search_aux = aux.clone())
        ),
        Buscando::False { pos, focus } => view!(cx,
          CuadroPrincipal(pos= pos.clone())
        ),
    })
      ResumenPago(venta=venta.clone(),pos=match buscando.get().as_ref().clone(){
        Buscando::False { pos, focus } => pos.clone(),
        Buscando::True { pos, .. } => pos.clone(),
    },
      config=config.clone())
    )
}
