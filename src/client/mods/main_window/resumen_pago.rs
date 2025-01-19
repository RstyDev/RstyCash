use crate::client::mods::{main_window::pagos::Pagos, structs::{Config, Pos, Valuable, Venta}};
use sycamore::{prelude::*,reactive::{create_signal, Signal}};

#[derive(Props)]
pub struct ResumenProps {
    pub venta: Signal<Venta>,
    pub config: Signal<Config>,
    pub pos: Signal<Pos>,
    pub other_sale: Signal<Venta>,
    pub focus: Signal<bool>,
}
#[allow(non_snake_case)]
#[component]
pub fn ResumenPago(props: ResumenProps) -> View {
    let class = create_signal(match props.focus.with(|f| *f) {
        true => "not-focused",
        false => "",
    });
    create_memo(move || {
        class.set(match props.focus.get() {
            true => "not-focused",
            false => "",
        })
    });
    let prods = create_signal(props.venta.with(|v| v.productos.clone()));
    let a_pagar = create_signal(props.venta.with(|v| v.monto_total - v.monto_pagado));
    let format = props.config.with(|c| c.formato_producto);
    create_memo(move || {
        a_pagar.set(props.venta.with(|v| v.monto_total - v.monto_pagado));
    });
    view!(
        aside(id="resumen-y-pago", class=format!("focuseable {}",class.get())){
            article(){
                Keyed(
                    list = prods,
                    view=move |prod|{
                        view!{
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
                Pagos(venta=props.venta,config=props.config,pos=props.pos,other_sale=props.other_sale,focus=props.focus)
                p(){
                    (format!("Total a pagar:  {:.2}",a_pagar.get()))
                }
            }
        }
    )
}
