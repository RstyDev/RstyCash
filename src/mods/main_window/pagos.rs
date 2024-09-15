use super::resumen_pago::ResumenProps;
use crate::mods::{
    main_window::*,
    structs::{Cliente, Cuenta, MedioPago},
};
use pago::*;
use sycamore::{
    prelude::*,
    reactive::{create_memo, create_rc_signal},
};

#[component]
pub fn Pagos<G: Html>(cx: Scope, props: ResumenProps) -> View<G> {
    let (venta, venta1) = (props.venta.clone(), props.venta.clone());
    let conf = props.config.clone();
    let pagos = create_signal(cx, venta.get().pagos.clone());
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
        })
    });
    let state = create_rc_signal(String::new());
    view!(cx,
        article(id="pagos"){
            Keyed(
                iterable = pagos,
                view=move |cx,x|view!(cx,
                    PagoComp(pagado = true, opciones = create_rc_signal(vec![x.medio_pago]), monto = x.monto, state = None)
                ),
                key=|x|x.int_id
            )
            PagoComp(pagado=false, opciones=medios.clone(), monto=venta1.get().monto_total - venta1.get().monto_pagado, state=Some(state))
        }
    )
}
