use crate::client::mods::{
    main_window::productos::*,
    structs::{Config, Venta},
};
use sycamore::prelude::*;

#[derive(Props)]
pub struct VentaProps {
    pub venta: Signal<Venta>,
    pub config: Signal<Config>,
    pub pos: bool,
    pub focus: Signal<bool>,
}
#[allow(non_snake_case)]
#[component]
pub fn CuadroVenta(props: VentaProps) -> View {

    view! {
        section(id="cuadro-venta"){
            Productos(venta=props.venta.clone(),config=props.config.clone(), pos = props.pos,focus=props.focus.clone())
            section(id="monto-total"){(format!("TOTAL {:.2}",props.venta.with(|v|v.monto_total)))}
        }
    }
}
