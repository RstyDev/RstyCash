use crate::client::mods::{
    lib::{call, debug},
    structs::{
        args::{AgregarPago, EliminarPago},
        MedioPago, Pago, Pos, Restante, Venta, VentaSH,
    },
};
use serde_wasm_bindgen::from_value;
use sycamore::{futures::spawn_local_scoped, prelude::*, reactive::Signal, Props};
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, SubmitEvent};

#[derive(Props)]
pub struct PagoProps {
    opciones: Signal<Vec<MedioPago>>,
    pago: Pago,
    monto: Restante,
    pos: Signal<Pos>,
    other_sale: Signal<Venta>,
    focus: Signal<bool>,
}
#[allow(non_snake_case)]
#[component]
pub fn PagoComp(props: PagoProps) -> View {
    let opcion = create_signal(props.opciones.with(|o| o[0].medio.to_string()));
    let monto = create_signal(String::new());
    let enter = create_signal(false);
    let borrar = create_signal(false);
    let aux_monto = props.monto.clone();
    let aux_focus = props.focus.clone();
    // create_memo(move || {
    //     props.opciones.track();
    //     opts.set(props.opciones.get_clone())
    // });
    let aux_pago = props.pago.clone();
    create_memo(move || match &aux_monto {
        Restante::Pagado(_) => (),
        Restante::NoPagado(rc_signal) => {
            rc_signal.track();
            monto.set(String::new());
        }
    });
    create_effect(move || {
        if borrar.get() {
            let pos = props.pos.with(|p| p.is_a());
            let pago = aux_pago.clone();
            spawn_local_scoped(async move {
                let res = call("eliminar_pago", EliminarPago { pago, pos }).await;
                props.pos.with(|pos| match pos {
                    Pos::A { venta, .. } => {
                        venta.set(Venta::from_shared(from_value::<VentaSH>(res).unwrap()))
                    }
                    Pos::B { venta, .. } => {
                        venta.set(Venta::from_shared(from_value::<VentaSH>(res).unwrap()))
                    }
                });
            });
        }
    });
    let aux_pago = props.pago.clone();
    create_effect(move || {
        if enter.get() {
            let (pos, venta) = props.pos.with(|p| match p {
                Pos::A { venta, .. } => (true, venta.clone()),
                Pos::B { venta, .. } => (false, venta.clone()),
            });
            let mut pago = aux_pago.clone();
            let monto = monto.with(|m| m.parse::<f32>().unwrap());
            pago.pagado = if opcion.with(|o| o.eq("Cuenta Corriente")) {
                0.0
            } else {
                monto
            };
            pago.medio_pago = props.opciones.with(|o| {
                o.iter()
                    .find(|&m| m.medio.as_str().eq(&opcion.get_clone()))
                    .unwrap()
                    .clone()
            });
            pago.monto = monto;
            let prop_pos = props.pos.clone();
            let other_sale = props.other_sale.clone();
            spawn_local_scoped(async move {
                let len_anterior = venta.with(|v| v.productos.len());
                let res = call("agregar_pago", AgregarPago { pago, pos }).await;
                let venta_nueva = Venta::from_shared(from_value::<VentaSH>(res.clone()).unwrap());
                let len = venta_nueva.productos.len();
                if len == 0 && len_anterior > 0 {
                    debug(&prop_pos.get_clone(), 80, "Pago");
                    prop_pos.set(prop_pos.with(|p| match p {
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
                    }));
                }
                venta.set(venta_nueva);
            });
        }
    });
    let aux_monto = props.monto.clone();
    let aux_monto2 = props.monto.clone();
    let aux_monto3 = props.monto.clone();
    let aux_monto4 = props.monto.clone();
    let aux_monto5 = props.monto.clone();
    view! {
        form(id="form-pago"){
            input(r#type="number",placeholder=props.monto.clone().to_string(),class="input-monto",disabled = aux_monto2.clone().pagado(),bind:value=monto, on:keyup=move |e:KeyboardEvent|{
                let event: KeyboardEvent = e.clone().unchecked_into();
                if event.key().eq("Enter")&&!monto.with(|m|m.eq(""))&&monto.with(|m|m.parse::<f32>().unwrap())!=0.0{
                    e.prevent_default();
                    enter.set(true);
                    enter.set(false);
                }
            },on:focus=move |_|{
                if !aux_monto4.pagado() && props.focus.get(){
                    props.focus.set(false)
                }
            })
            select(class="opciones-pagos",disabled = aux_monto.clone().pagado(), bind:value=opcion, tabindex="-1",on:focus=move |_|{
                if !aux_monto5.pagado() && props.focus.get(){
                    props.focus.set(false)
                }
            }){
            Keyed(
                list = props.opciones,
                view=|x|view!{
                    option(){(x.medio)}
                },
                key=|x|x.id,
            )
        }
            input(r#type="submit", value=match aux_monto3{Restante::Pagado(_) => "Borrar",Restante::NoPagado(_) => "Pagar"}, on:submit=move |a:SubmitEvent|{
                a.prevent_default();
                if props.focus.get(){
                    props.focus.set(false);
                }
                match aux_monto3{
                    Restante::Pagado(_) => {
                        borrar.set(true);
                        borrar.set(false);
                    },
                    Restante::NoPagado(_) => {
                            if !monto.with(|m|m.eq("")){
                            enter.set(true);
                            enter.set(false);
                        }
                    },
                }
            })
        }
    }
}
