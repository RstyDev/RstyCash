use serde_wasm_bindgen::from_value;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::{create_effect, create_memo, create_signal, create_signal_from_rc};
use sycamore::rt::Event;
use sycamore::{
    prelude::{component, view, Html, Keyed, Prop, Scope, View},
    reactive::RcSignal,
};
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

use crate::mods::lib::call;
use crate::mods::structs::args::AgregarPago;
use crate::mods::structs::{MedioPago, Pago, Pos, Restante, Venta, VentaSHC};

#[derive(Prop)]
pub struct PagoProps {
    pagado: bool,
    opciones: RcSignal<Vec<MedioPago>>,
    monto: Restante,
    state: Option<RcSignal<String>>,
    pos: RcSignal<Pos>,
}

#[component]
pub fn PagoComp<G: Html>(cx: Scope, props: PagoProps) -> View<G> {
    let opts = create_signal_from_rc(cx, props.opciones.get());
    let restante = props.monto.clone();
    let rest1 = props.monto.clone();
    let opcion = create_signal(cx, props.opciones.get().as_ref()[0].medio.to_string());
    let monto = create_signal(cx, String::new());
    let enter = create_signal(cx, false);

    create_memo(cx, move || {
        props.opciones.track();
        opts.set(props.opciones.get().as_ref().clone())
    });

    create_memo(cx, move || match &rest1 {
        Restante::Pagado(_) => (),
        Restante::NoPagado(rc_signal) => {
            rc_signal.track();
            monto.set(String::new());
        }
    });

    create_effect(cx, move || {
        if *enter.get().as_ref() {
            let (pos, venta) = match props.pos.get().as_ref() {
                Pos::A { venta, .. } => (true, venta.clone()),
                Pos::B { venta, .. } => (false, venta.clone()),
            };
            let monto = monto.get().parse::<f32>().unwrap();
            let medio_pago = opts
                .get()
                .iter()
                .find(|m| m.medio.as_ref().eq(opcion.get().as_ref()))
                .unwrap()
                .clone();
            let pago = Pago {
                int_id: 0,
                medio_pago,
                monto,
                pagado: if opcion.get().as_ref().eq("Cuenta Corriente") {
                    0.0
                } else {
                    monto
                },
            };

            spawn_local_scoped(cx, async move {
                let res = call("agregar_pago", AgregarPago { pago, pos }).await;
                venta.set(Venta::from_shared_complete(
                    from_value::<VentaSHC>(res.clone()).unwrap(),
                ));
            });
        }
    });
    view! {cx,
        form(id="form-pago"){
            input(type="number",placeholder=restante.to_string(),class="input-monto",disabled = props.pagado,bind:value=monto, on:keyup=|e:Event|{
                let event: KeyboardEvent = e.clone().unchecked_into();
                if event.key().eq("Enter"){
                    e.prevent_default();
                    enter.set(true);
                    enter.set(false);
                }
            })
            select(class="opciones-pagos",disabled = props.pagado, bind:value=opcion){
            Keyed(
                iterable = opts,
                view=|cx,x|view!{cx,
                    option(){(x.medio)}
                },
                key=|x|x.id,
            )
        }
            input(type="submit", value=match props.pagado{true => "Borrar",false => "Pagar"}, on:click=move |a:Event|{
                a.prevent_default();
                match &props.state{
                   Some(s) => s.set(String::from("Desde Pago")),
                   None => (),
                }
            })
        }
    }
}
