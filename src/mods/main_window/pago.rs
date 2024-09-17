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
use crate::mods::structs::args::AgregarPago;
use crate::mods::structs::{MedioPago, Pago, Pos, Venta, VentaSHC};

#[derive(Prop)]
pub struct PagoProps {
    pagado: bool,
    opciones: RcSignal<Vec<MedioPago>>,
    monto: f32,
    state: Option<RcSignal<String>>,
    pos: RcSignal<Pos>,
    pago: Pago,
}

#[component]
pub fn PagoComp<G: Html>(cx: Scope, props: PagoProps) -> View<G> {
    let opts = create_signal_from_rc(cx, props.opciones.get());
    let opcion = create_signal(cx, props.opciones.get().as_ref()[0].medio.to_string());
    let monto = create_signal(cx,String::new());
    let enter = create_signal(cx, false);

    create_memo(cx, move || {
        props.opciones.track();
        opts.set(props.opciones.get().as_ref().clone())
    });

    create_effect(cx, move ||{
        if *enter.get().as_ref(){
            let (pos,venta)= match props.pos.get().as_ref(){
                Pos::A { venta,.. } => (true,venta.clone()),
                Pos::B { venta,.. } => (false,venta.clone()),
            };
            let pago = props.pago.clone();
            spawn_local_scoped(cx, async move{
                let res=call("agregar_pago", AgregarPago{ pago: pago, pos: pos }).await;
                match from_value::<VentaSHC>(res){
                    Ok(a) => venta.set(Venta::from_shared_complete(a)),
                    Err(e) => debug(e,49,"pago"),
                }
                
            });
            debug(monto.get(), 38, "pago");
            debug(opcion.get(), 38, "pago");
        }
    });
    view! {cx,
        form(id="form-pago"){
            input(type="number",placeholder=props.monto,class="input-monto",disabled = props.pagado,bind:value=monto, on:keyup=|e:Event|{
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
