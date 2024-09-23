use super::resumen_pago::ResumenProps;
use crate::mods::{
    main_window::*,
    structs::{Cliente, Cuenta, MedioPago, Restante},
};
use pago::*;
use sycamore::{
    prelude::*,
    reactive::{create_memo, create_rc_signal},
};

#[component]
pub fn Pagos<G: Html>(cx: Scope, props: ResumenProps) -> View<G> {
    let other_sale = props.other_sale.clone();
    let other_sale2 = props.other_sale.clone();
    let (venta, venta2) = (props.venta.clone(), props.venta.clone());
    let conf = props.config.clone();
    let restante = create_rc_signal(venta.get().monto_total - venta.get().monto_pagado);
    let rest1 = restante.clone();
    create_memo(cx, move || {
        let venta = venta2.get();
        rest1.set(venta.monto_total - venta.monto_pagado);
    });
    let pagos = create_signal(cx, venta.get().pagos.clone());
    let (pos, pos1) = (props.pos.clone(), props.pos.clone());
    let medios = create_rc_signal({
        let filtrado = conf
            .get()
            .medios_pago
            .iter()
            .cloned()
            .filter(|m| m.id != 0)
            .collect::<Vec<MedioPago>>();
        match venta.get().cliente.clone() {
            Cliente::Final => filtrado,
            Cliente::Regular(cli) => match cli.limite {
                Cuenta::Auth(_) => conf.get().medios_pago.clone(),
                Cuenta::Unauth => filtrado,
            },
        }
    });
    let medios2 = medios.clone();
    create_memo(cx, move || {
        venta.track();
        let filtrado = conf
            .get()
            .medios_pago
            .iter()
            .cloned()
            .filter(|m| m.id != 0)
            .collect::<Vec<MedioPago>>();
        medios2.set(match venta.get().cliente.clone() {
            Cliente::Final => filtrado,
            Cliente::Regular(cli) => match cli.limite {
                Cuenta::Auth(_) => conf.get().medios_pago.clone(),
                Cuenta::Unauth => filtrado,
            },
        });
        pagos.set(venta.get().pagos.clone());
    });
    let state = create_rc_signal(String::new());
    view!(cx,
        article(id="pagos"){
            Keyed(
                iterable = pagos,
                view=move |cx,x|{
                    let pos = pos.clone();
                    let other_sale=other_sale.clone();
                    view!(cx,
                        PagoComp(pagado = true, opciones = create_rc_signal(vec![x.medio_pago.clone()]), monto = Restante::Pagado(x.monto), state = None, pos = pos.clone(),other_sale=other_sale.clone())
                    )
                },
                key=|x|x.int_id
            )
            PagoComp(pagado=false, opciones=medios.clone(), monto=Restante::NoPagado(restante.clone()), state=Some(state), pos = pos1.clone(),other_sale=other_sale2.clone())
        }
    )
}
