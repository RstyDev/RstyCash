use crate::client::mods::{main_window::cuadro_venta::*, structs::Pos};
use sycamore::prelude::*;
#[derive(Prop)]
pub struct PrincProps {
    pub pos: RcSignal<Pos>,
    pub focus: RcSignal<bool>,
}
#[allow(non_snake_case)]
#[component]
pub fn CuadroPrincipal<G: Html>(cx: Scope, props: PrincProps) -> View<G> {
    let foc = props.focus.clone();
    view! {cx,
        section(id="cuadro-principal",class=format!("focuseable {}",match foc.get().as_ref(){
            true => "",
            false => "not-focused",
        })){
            (match props.pos.get().as_ref(){
                Pos::A { venta, config, .. } => {
                    let foc = props.focus.clone();
                    view!{cx,CuadroVenta(venta=venta.clone(),config=config.get(),pos=true,focus=foc.clone())}
                },
                Pos::B { venta, config, .. } => {
                    let foc = props.focus.clone();
                    view!{cx,CuadroVenta(venta=venta.clone(),config=config.get(),pos=false,focus=foc.clone())}
                },
            })
        }
    }
}
