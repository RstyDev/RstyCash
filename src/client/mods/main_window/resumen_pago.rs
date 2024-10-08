use crate::client::mods::main_window::pagos::Pagos;
use crate::client::mods::structs::{Config, Pos, Valuable, Venta};
use sycamore::flow::Keyed;
use sycamore::prelude::{component, create_memo, view, Html, Prop, Scope, View};
use sycamore::reactive::{create_rc_signal, create_signal, RcSignal};
#[derive(Prop)]
pub struct ResumenProps {
    pub venta: RcSignal<Venta>,
    pub config: RcSignal<Config>,
    pub pos: RcSignal<Pos>,
    pub other_sale: RcSignal<Venta>,
    pub focus: RcSignal<bool>,
}
#[allow(non_snake_case)]
#[component]
pub fn ResumenPago<G: Html>(cx: Scope, props: ResumenProps) -> View<G> {
    let venta = props.venta.clone();
    let venta1 = venta.clone();
    let class = create_signal(
        cx,
        match *props.focus.get() {
            true => "not-focused",
            false => "",
        },
    );
    let foc = props.focus.clone();
    create_memo(cx, move || {
        class.set(match *foc.get() {
            true => "not-focused",
            false => "",
        })
    });
    let prods = create_signal(cx, venta.get().productos.clone());
    let a_pagar = create_rc_signal(venta.get().monto_total - venta.get().monto_pagado);
    let a_pagar1 = a_pagar.clone();
    let format = props.config.get().formato_producto;
    create_memo(cx, move || {
        a_pagar1.set(venta1.get().monto_total - venta1.get().monto_pagado);
    });
    view!(cx,
        aside(id="resumen-y-pago", class=format!("focuseable {}",class.get())){
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
            section(id="section-pagos"){
                Pagos(props)
                p(){
                    (format!("Total a pagar:  {:.2}",a_pagar.get()))
                }
            }
        }
    )
}
