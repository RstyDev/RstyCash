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

use crate::mods::lib::{call, debug};
use crate::mods::structs::args::{AgregarPago, EliminarPago};
use crate::mods::structs::{MedioPago, Pago, Pos, Restante, Venta, VentaSHC};

#[derive(Prop)]
pub struct PagoProps {
    opciones: RcSignal<Vec<MedioPago>>,
    pago: Pago,
    monto: Restante,
    pos: RcSignal<Pos>,
    other_sale: RcSignal<Venta>,
    focus: RcSignal<bool>,
}
#[allow(non_snake_case)]
#[component]
pub fn PagoComp<G: Html>(cx: Scope, props: PagoProps) -> View<G> {
    let opts = create_signal_from_rc(cx, props.opciones.get());
    let pos = props.pos.clone();
    let restante = props.monto.clone();
    let rest1 = props.monto.clone();
    let (pago1,pago2)=(props.pago.clone(),props.pago.clone());
    let (rest2,rest3,rest4,rest5,rest6) = (props.monto.clone(),props.monto.clone(),props.monto.clone(),props.monto.clone(),props.monto.clone());
    let opcion = create_signal(cx, props.opciones.get().as_ref()[0].medio.to_string());
    let monto = create_signal(cx, String::new());
    let enter = create_signal(cx, false);
    let borrar = create_signal(cx, false);
    let focus = props.focus.clone();
    let focus1 = props.focus.clone();
    let focus2 = props.focus.clone();
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
        if *borrar.get() {
            let pos1 = pos.clone();
            let pos=pos.get().is_a();
            let pago = pago2.clone();
            spawn_local_scoped(cx,async move {
                let res = call("eliminar_pago",EliminarPago{pago,pos}).await;
                match pos1.get().as_ref(){
                    Pos::A { venta, .. } => venta.set(Venta::from_shared_complete(from_value::<VentaSHC>(res).unwrap())),
                    Pos::B { venta, .. } => venta.set(Venta::from_shared_complete(from_value::<VentaSHC>(res).unwrap())),
                }
            });
        }
    });
    create_effect(cx, move || {
        if *enter.get().as_ref() {
            let (pos, venta) = match props.pos.get().as_ref() {
                Pos::A { venta, .. } => (true, venta.clone()),
                Pos::B { venta, .. } => (false, venta.clone()),
            };
            let mut pago = pago1.clone();
            let monto = monto.get().parse::<f32>().unwrap();
            pago.pagado = if opcion.get().as_ref().eq("Cuenta Corriente"){0.0}else{monto};
            pago.medio_pago = opts
            .get()
            .iter()
            .find(|m| m.medio.as_ref().eq(opcion.get().as_ref()))
            .unwrap()
            .clone();
            pago.monto = monto;
            let prop_pos = props.pos.clone();
            let other_sale = props.other_sale.clone();
            spawn_local_scoped(cx, async move {
                let len_anterior = venta.get().as_ref().productos.len();
                let res = call("agregar_pago", AgregarPago { pago, pos }).await;
                let venta_nueva =
                    Venta::from_shared_complete(from_value::<VentaSHC>(res.clone()).unwrap());
                let len = venta_nueva.productos.len();
                if len == 0 && len_anterior > 0 {
                    debug(prop_pos.get().as_ref(), 80, "Pago");
                    prop_pos.set(match prop_pos.get().as_ref() {
                        Pos::A {
                            venta: _,
                            config,
                            clientes,
                        } => Pos::B {
                            venta: other_sale.clone(),
                            config: config.to_owned(),
                            clientes: clientes.to_owned(),
                        },
                        Pos::B {
                            venta: _,
                            config,
                            clientes,
                        } => Pos::A {
                            venta: other_sale.clone(),
                            config: config.to_owned(),
                            clientes: clientes.to_owned(),
                        },
                    });
                }
                venta.set(venta_nueva);
            });
        }
    });
    view! {cx,
        form(id="form-pago"){
            input(type="number",placeholder=restante.to_string(),class="input-monto",disabled = rest2.pagado(),bind:value=monto, on:keyup=|e:Event|{
                let event: KeyboardEvent = e.clone().unchecked_into();
                if event.key().eq("Enter")&&!monto.get().as_ref().eq("")&&monto.get().as_ref().parse::<f32>().unwrap()!=0.0{
                    e.prevent_default();
                    enter.set(true);
                    enter.set(false);
                }
            },on:focus=move |_|{
                if !rest3.pagado() && *focus.get(){
                    focus.set(false)
                }
            })
            select(class="opciones-pagos",disabled = rest4.pagado(), bind:value=opcion, tabindex="-1",on:focus=move |_|{
                if !rest5.pagado() && *focus1.get(){
                    focus1.set(false)
                }
            }){
            Keyed(
                iterable = opts,
                view=|cx,x|view!{cx,
                    option(){(x.medio)}
                },
                key=|x|x.id,
            )
        }
            input(tabindex="-1",type="submit", value=match rest6{Restante::Pagado(_) => "Borrar",Restante::NoPagado(_) => "Pagar"}, on:click=move |a:Event|{
                a.prevent_default();
                if *focus2.get(){
                    focus2.set(false);
                }
                match props.monto{
                    Restante::Pagado(_) => {
                        borrar.set(true);
                        borrar.set(false);
                    },
                    Restante::NoPagado(_) => {
                            if !monto.get().as_ref().eq(""){
                            enter.set(true);
                            enter.set(false);
                        }
                    },
                }
            })
        }
    }
}
