use std::rc::Rc;

use crate::client::mods::{
    lib::{call, debug},
    structs::{
        args::{
            DecrementarProductoDeVenta, EliminarProductoDeVenta, IncrementarProductoAVenta,
            SetCantidadProductoVenta,
        },
        Config, Valuable, Venta, VentaSHC,
    },
};
use serde_wasm_bindgen::from_value;
use sycamore::{
    futures::spawn_local_scoped,
    prelude::{component, create_effect, create_selector, view, Html, Prop, RcSignal, Scope, View},
    reactive::{create_signal, create_signal_from_rc},
};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::Event;

#[derive(Prop)]
pub struct ProdProps {
    venta: RcSignal<Venta>,
    valuable: Rc<Valuable>,
    conf: Rc<Config>,
    pos: bool,
    focus: RcSignal<bool>,
}
#[allow(non_snake_case)]
#[component]
pub fn Prod<G: Html>(cx: Scope, props: ProdProps) -> View<G> {
    let conf = create_signal_from_rc(cx, props.conf);
    let cantidad = create_signal(cx, props.valuable.get_f_cant().to_string());
    let desc = create_signal(cx, props.valuable.get_desc(conf.get().formato_producto));
    let cambio = create_signal(cx, false);
    let val = props.valuable.clone();
    let val1 = props.valuable.clone();
    let val2 = props.valuable.clone();
    let val3 = props.valuable.clone();
    let val5 = props.valuable.clone();
    let val6 = props.valuable.clone();
    let disabled = create_selector(cx, move || val6.get_f_cant() <= 1.0);
    let rc_venta = props.venta.clone();
    let rc_venta1 = props.venta.clone();
    let rc_venta2 = props.venta.clone();
    let rc_venta3 = props.venta.clone();
    let focus = move |_| {
        debug(&"aca", 94, "producto");
        props.focus.set(true)
    };
    let (foc1, foc2, foc3) = (focus.clone(), focus.clone(), focus.clone());
    create_effect(cx, move || {
        if *cambio.get() {
            let rc_venta3 = rc_venta3.clone();
            let val3 = val3.clone();
            spawn_local_scoped(cx, async move {
                if cantidad.get().as_str() == "" {
                    let pos = props.pos;
                    let res = call(
                        "eliminar_producto_de_venta",
                        EliminarProductoDeVenta {
                            index: rc_venta3
                                .get()
                                .as_ref()
                                .clone()
                                .productos
                                .iter()
                                .enumerate()
                                .find(|v| v.1 == val3.as_ref())
                                .unwrap()
                                .0,
                            pos,
                        },
                    )
                    .await;
                    let venta = from_value::<VentaSHC>(res).unwrap();
                    rc_venta3.set(Venta::from_shared_complete(venta));
                } else {
                    let res = call(
                        "set_cantidad_producto_venta",
                        SetCantidadProductoVenta {
                            index: rc_venta3
                                .get()
                                .as_ref()
                                .clone()
                                .productos
                                .iter()
                                .enumerate()
                                .find(|v| v.1 == val3.as_ref())
                                .unwrap()
                                .0,
                            cantidad: cantidad.get().parse::<f32>().unwrap(),
                            pos: props.pos,
                        },
                    )
                    .await;
                    let venta = Venta::from_shared_complete(from_value::<VentaSHC>(res).unwrap());
                    rc_venta3.set(venta);
                }
            });
        }
    });
    view!(cx,
        article(class="articulo",on:focus=focus.clone()){
            section(class=format!("descripcion {}",conf.get().modo_mayus)){
                p(){(desc.get())}
            }
            section(class="cantidad"){
                button(tabindex="-1",class=match disabled.get().as_ref(){
                    false => "button restar",
                    true => "button restar disabled",
                },on:click = move |x|{
                    let val = val5.clone();
                    focus(x);
                    let pos = props.pos;
                    let rc_venta = rc_venta2.clone();
                    spawn_local(async move{
                        let res = call("descontar_producto_de_venta", DecrementarProductoDeVenta{ index: rc_venta.get()
                            .as_ref()
                            .clone()
                            .productos
                            .iter()
                            .enumerate()
                            .find(|v| v.1 == val.as_ref())
                            .unwrap()
                            .0, pos }).await;
                        let venta = from_value::<VentaSHC>(res).unwrap();
                        rc_venta.set(Venta::from_shared_complete(venta));
                    })
                },disabled=*disabled.get().as_ref()){"-"}
                input(type="number",class="cantidad-producto",on:focus=foc1,bind:value=cantidad,on:keyup=|e:Event|{
                    let event:web_sys::KeyboardEvent = e.clone().unchecked_into();
                    if event.key().eq("Enter"){
                        cambio.set(true);
                        cambio.set(false);
                    }
                })
                button(tabindex="-1",class="button sumar",on:focus=foc2,on:click=move|_|{
                    let val = val1.clone();
                    let pos = props.pos;
                    let rc_venta = rc_venta1.clone();
                    spawn_local(async move{
                        let res=call("incrementar_producto_a_venta", IncrementarProductoAVenta { index: rc_venta.get()
                            .as_ref()
                            .clone()
                            .productos
                            .iter()
                            .enumerate()
                            .find(|v| v.1 == val.as_ref())
                            .unwrap()
                            .0, pos }).await;
                        let venta=from_value::<VentaSHC>(res).unwrap();
                        rc_venta.set(Venta::from_shared_complete(venta));
                    })
                }){"+"}
            }
            section(class="monto"){
                p(){
                    (format!("{:.2}",props.valuable.get_unit_price()))
                }
            }
            section(){
                p(){
                    (format!("{:.2}",val.get_total_price()))
                }
            }
            section(id="borrar"){
                button(tabindex="-1",class="button eliminar",on:click=move |_|{
                    let val = val2.clone();
                    let pos = props.pos;
                    let rc_venta = rc_venta.clone();
                    spawn_local(async move{
                        let res=call("eliminar_producto_de_venta", EliminarProductoDeVenta { index: rc_venta.get()
                            .as_ref()
                            .clone()
                            .productos
                            .iter()
                            .enumerate()
                            .find(|v| v.1 == val.as_ref())
                            .unwrap()
                            .0, pos }).await;
                        let venta=from_value::<VentaSHC>(res).unwrap();
                        rc_venta.set(Venta::from_shared_complete(venta));
                    })
                },on:focus=foc3){"Borrar"}
            }
        }
    )
}
