use crate::mods::{
    main_window::cuadro_venta::*,
    structs::{Config, Pos, Venta},
};
use sycamore::prelude::*;
#[derive(Prop)]
pub struct PrincProps {
    pub venta: RcSignal<Venta>,
    pub config: RcSignal<Config>,
    pub pos: RcSignal<Pos>,
}

#[component]
pub fn CuadroPrincipal<G: Html>(cx: Scope, props: PrincProps) -> View<G> {
    view! {cx,
        section(id="cuadro-principal"){
            CuadroVenta(venta=props.venta.clone(),config=props.config.get(),pos = match props.pos.get().as_ref(){
                Pos::A { venta:_, config:_, clientes:_ } => true,
                Pos::B { venta:_, config:_, clientes:_ } => false,
            })
        }
    }
}
