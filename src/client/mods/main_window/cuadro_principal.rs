use crate::client::mods::{main_window::cuadro_venta::*, structs::Pos};
use sycamore::prelude::*;
#[derive(Props)]
pub struct PrincProps {
    pub pos: Signal<Pos>,
    pub focus: Signal<bool>,
}
#[allow(non_snake_case)]
#[component]
pub fn CuadroPrincipal(props: PrincProps) -> View {
    view! {
        section(id="cuadro-principal",class=format!("focuseable {}",match props.focus.get(){
            true => "",
            false => "not-focused",
        })){
            (props.pos.with(|p|match p{
                Pos::A { venta, config, .. } => {
                    view!{CuadroVenta(venta=venta.clone(),config=config.clone(),pos=true,focus=props.focus.clone())}
                },
                Pos::B { venta, config, .. } => {
                    view!{CuadroVenta(venta=venta.clone(),config=config.clone(),pos=false,focus=props.focus.clone())}
                },
            }))
        }
    }
}
