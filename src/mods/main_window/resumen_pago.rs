use std::rc::Rc;
use std::sync::Arc;
use sycamore::flow::Keyed;
use sycamore::prelude::{component, view, Html, Prop, Scope, View};
use sycamore::reactive::{
    create_rc_signal, create_signal, create_signal_from_rc, RcSignal, Signal,
};

use crate::mods::main_window::pagos::Pagos;
use crate::mods::structs::{Config, Valuable, Venta};
#[derive(Prop)]
pub struct ResumenProps {
    pub venta: RcSignal<Venta>,
    pub config: RcSignal<Config>,
}
impl ResumenProps {
    pub fn new(venta: RcSignal<Venta>, config: RcSignal<Config>) -> ResumenProps {
        ResumenProps { venta, config }
    }
}

#[component]
pub fn ResumenPago<G: Html>(cx: Scope, props: ResumenProps) -> View<G> {
    let venta = props.venta.clone();

    let prods = create_signal(cx, venta.get().productos.clone());
    let a_pagar = create_rc_signal(venta.get().monto_total - venta.get().monto_pagado);
    let format = props.config.get().formato_producto;
    view!(cx,
        aside(id="resumen-y-pago"){
            article(){
                Keyed(
                    iterable = prods,
                    view=move |cx,prod|{
                        view!{cx,
                            p(){
                                (prod.get_desc(format))
                            }
                        }
                    },
                    key= |x|{
                        match x{
                            Valuable::Prod((_,prod)) => prod.codigos_de_barras[0],
                            Valuable::Pes((_,pes)) => pes.codigo,
                            Valuable::Rub((_,rub)) => rub.codigo,
                        }
                    }
                )
            }
            section(){
                Pagos(props)
                p(){
                    "Total a pagar: " (a_pagar)
                }
            }
        }
    )
}
