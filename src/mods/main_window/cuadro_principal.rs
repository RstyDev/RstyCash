use crate::mods::{main_window::cuadro_venta::*, structs::Pos};
use sycamore::prelude::*;
#[derive(Prop)]
pub struct PrincProps {
    pub pos: RcSignal<Pos>,
    pub focus: RcSignal<bool>
}

#[component]
pub fn CuadroPrincipal<G: Html>(cx: Scope, props: PrincProps) -> View<G> {
    let foc=props.focus.clone();
    view! {cx,
        section(id="cuadro-principal",class=format!("focuseable {}",match foc.get().as_ref(){
            true => "",
            false => "not-focused",
        })){
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
