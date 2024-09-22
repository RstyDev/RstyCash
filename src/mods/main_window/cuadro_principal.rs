use crate::mods::{lib::debug, main_window::cuadro_venta::*, structs::Pos};
use sycamore::prelude::*;
#[derive(Prop)]
pub struct PrincProps {
    pub pos: RcSignal<Pos>,
}

#[component]
pub fn CuadroPrincipal<G: Html>(cx: Scope, props: PrincProps) -> View<G> {
    view! {cx,
        section(id="cuadro-principal"){
            (match props.pos.get().as_ref(){
                Pos::A { venta, config, .. } => view!{cx,CuadroVenta(venta=venta.clone(),config=config.get(),pos=true)},
                Pos::B { venta, config, .. } => view!{cx,CuadroVenta(venta=venta.clone(),config=config.get(),pos=false)},
            })
            // CuadroVenta(venta=props.venta.clone(),config=props.config.get(),pos = match props.pos.get().as_ref(){
            //     Pos::A { .. } => true,
            //     Pos::B { .. } => false,
            // })
        }
    }
}
