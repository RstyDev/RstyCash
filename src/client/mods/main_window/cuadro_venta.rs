use crate::client::mods::{
    main_window::productos::*,
    structs::{Config, Venta},
};
use std::rc::Rc;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct VentaProps {
    pub venta: RcSignal<Venta>,
    pub config: Rc<Config>,
    pub pos: bool,
    pub focus: RcSignal<bool>,
}
#[allow(non_snake_case)]
#[component]
pub fn CuadroVenta<G: Html>(cx: Scope, props: VentaProps) -> View<G> {
    let venta = props.venta.clone();
    let venta1 = props.venta.clone();
    let config = create_signal_from_rc(cx, props.config);
    view! {cx,
        section(id="cuadro-venta"){
            Productos(venta=venta.clone(),config=config.get(), pos = props.pos,focus=props.focus.clone())
            section(id="monto-total"){(format!("TOTAL {:.2}",venta1.get().monto_total))}
        }
    }
}
