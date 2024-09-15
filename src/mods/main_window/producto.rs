use std::rc::Rc;

use crate::mods::{
    lib::call,
    structs::{
        args::{DecrementarProductoDeVenta, EliminarProductoDeVenta, IncrementarProductoAVenta},
        Config, Valuable, Venta, VentaSHC,
    },
};
use serde_wasm_bindgen::from_value;
use sycamore::{
    prelude::{component, create_selector, view, Html, Prop, RcSignal, Scope, View},
    reactive::{create_signal, create_signal_from_rc},
};
use wasm_bindgen_futures::spawn_local;

#[derive(Prop)]
pub struct ProdProps {
    venta: RcSignal<Venta>,
    valuable: Rc<Valuable>,
    conf: Rc<Config>,
    pos: bool,
}

#[component]
pub fn Prod<G: Html>(cx: Scope, props: ProdProps) -> View<G> {
    let conf = create_signal_from_rc(cx, props.conf);
    let desc = create_signal(cx, props.valuable.get_desc(conf.get().formato_producto));
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
    view!(cx,
        article(class="articulo"){
            section(class=format!("descripcion {}",conf.get().modo_mayus)){
                p(){(desc.get())}
            }
            section(class="cantidad"){
                button(class=match disabled.get().as_ref(){
                    false => "button restar",
                    true => "button restar disabled",
                },on:click = move |_|{
                    let val = val5.clone();
                    let pos = props.pos;
                    let rc_venta = rc_venta2.clone();
                    spawn_local(async move{
                        let res = call("descontar_producto_de_venta", DecrementarProductoDeVenta{ code: val.get_shared_code(), pos }).await;
                        let venta = from_value::<VentaSHC>(res).unwrap();
                        rc_venta.set(Venta::from_shared_complete(venta));
                    })
                },disabled=*disabled.get().as_ref()){"-"}
                input(type="text",class="cantidad-producto",value=val3.get_f_cant())
                button(class="button sumar",on:click=move|_|{
                    let val = val1.clone();
                    let pos = props.pos;
                    let rc_venta = rc_venta1.clone();
                    spawn_local(async move{
                        let res=call("incrementar_producto_a_venta", IncrementarProductoAVenta { code: val.get_shared_code(), pos }).await;
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
                button(class="button eliminar",on:click=move |_|{
                    let val = val2.clone();
                    let pos = props.pos;
                    let rc_venta = rc_venta.clone();
                    spawn_local(async move{
                        let res=call("eliminar_producto_de_venta", EliminarProductoDeVenta { code: val.get_shared_code(), pos }).await;
                        let venta=from_value::<VentaSHC>(res).unwrap();
                        rc_venta.set(Venta::from_shared_complete(venta));
                    })
                }){"Borrar"}
            }
        }
    )
}
