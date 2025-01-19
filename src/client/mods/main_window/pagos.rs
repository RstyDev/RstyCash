use super::resumen_pago::ResumenProps;
use crate::client::mods::{
    main_window::*,
    structs::{Cliente, Cuenta, MedioPago, Pago, Restante},
};
use pago::*;
use sycamore::{
    prelude::*,
    reactive::{create_memo, create_signal},
};
#[allow(non_snake_case)]
#[component]
pub fn Pagos(props: ResumenProps) -> View {
    let restante = create_signal(props.venta.with(|v| v.monto_total - v.monto_pagado));

    create_memo(move || {
        restante.set(props.venta.with(|v| v.monto_total - v.monto_pagado));
    });

    let pagos = create_signal(props.venta.with(|v| v.pagos.clone()));
    let medios = create_signal({
        let filtrado = props.config.with(|c| {
            c.medios_pago
                .iter()
                .cloned()
                .filter(|m| m.id != 0)
                .collect::<Vec<MedioPago>>()
        });

        props.venta.with(|v| match &v.cliente {
            Cliente::Final => filtrado,
            Cliente::Regular(cli) => match cli.limite {
                Cuenta::Auth(_) => props.config.with(|c| c.medios_pago.clone()),
                Cuenta::Unauth => filtrado,
            },
        })
    });
    create_memo(move || {
        let venta = props.venta.get_clone();
        let filtrado = props.config.with(|c| {
            c.medios_pago
                .iter()
                .cloned()
                .filter(|m| m.id != 0)
                .collect::<Vec<MedioPago>>()
        });

        medios.set(match venta.cliente.clone() {
            Cliente::Final => filtrado,
            Cliente::Regular(cli) => match cli.limite {
                Cuenta::Auth(_) => props.config.with(|c| c.medios_pago.clone()),
                Cuenta::Unauth => filtrado,
            },
        });
        pagos.set(venta.pagos.clone());
    });

    view!(
        article(id="pagos"){
            Keyed(
                list = pagos,
                view=move |x|{
                    view!(
                        PagoComp(pago=x.clone(),opciones = create_signal(vec![x.medio_pago.clone()]), monto = Restante::Pagado(x.monto), pos = props.pos.clone(),other_sale=props.other_sale.clone(),focus=props.focus.clone())
                    )
                },
                key=|x|x.int_id
            )
            PagoComp(pago=Pago::default(),opciones=medios.clone(), monto=Restante::NoPagado(restante.clone()), pos = props.pos.clone(),other_sale=props.other_sale.clone(),focus=props.focus.clone())
        }
    )
}
