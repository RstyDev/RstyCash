use crate::client::mods::{
    lib::{call, debug},
    structs::{
        args::{
            DecrementarProductoDeVenta, EliminarProductoDeVenta, IncrementarProductoAVenta,
            SetCantidadProductoVenta,
        },
        Config, Valuable, Venta, VentaSH,
    },
};
use serde_wasm_bindgen::from_value;
use std::rc::Rc;
use sycamore::{prelude::*, reactive::create_signal, Props};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::KeyboardEvent;

#[derive(Props)]
pub struct ProdProps {
    venta: Signal<Venta>,
    valuable: Valuable,
    conf: Signal<Config>,
    pos: bool,
    focus: Signal<bool>,
}
#[allow(non_snake_case)]
#[component]
pub fn Prod(props: ProdProps) -> View {
    let cantidad = create_signal(props.valuable.get_f_cant().to_string());
    let desc = create_signal(props.valuable.get_desc(props.conf.with(|c| c.formato_producto)));
    let cambio = create_signal(false);
    let valuable = props.valuable.clone();
    let disabled = create_selector(move || valuable.get_f_cant() <= 1.0);

    let focus = move |_| {
        debug(&"aca", 94, "producto");
        props.focus.set(true)
    };
    let (foc1, foc2, foc3) = (focus.clone(), focus.clone(), focus.clone());
    let valuable = props.valuable.clone();
    create_effect(move || {
        if cambio.get() {
            let venta = props.venta.clone();
            let val = valuable.clone();
            spawn_local(async move {
                if cantidad.with(|c| c.eq("")) {
                    let pos = props.pos;
                    let res = call(
                        "eliminar_producto_de_venta",
                        EliminarProductoDeVenta {
                            index: venta.with(|v| {
                                v.productos
                                    .iter()
                                    .enumerate()
                                    .find(|v| v.1 == &val)
                                    .unwrap()
                                    .0
                            }),
                            pos,
                        },
                    )
                    .await;
                    let venta_aux = from_value::<VentaSH>(res).unwrap();
                    venta.set(Venta::from_shared(venta_aux));
                } else {
                    let res = call(
                        "set_cantidad_producto_venta",
                        SetCantidadProductoVenta {
                            index: venta.with(|r| {
                                r.productos
                                    .iter()
                                    .enumerate()
                                    .find(|v| v.1 == &val)
                                    .unwrap()
                                    .0
                            }),
                            cantidad: cantidad.with(|c| c.parse::<f32>().unwrap()),
                            pos: props.pos,
                        },
                    )
                    .await;
                    let venta_aux = Venta::from_shared(from_value::<VentaSH>(res).unwrap());
                    venta.set(venta_aux);
                }
            });
        }
    });
    let valuable = props.valuable.clone();
    let valuable2 = props.valuable.clone();
    let valuable3 = props.valuable.clone();
    let valuable4 = props.valuable.clone();
    view!(
        article(class="articulo",on:focus=move |_|{props.focus.set(true)}){
            section(class=format!("descripcion {}",props.conf.with(|c|c.modo_mayus.clone()))){
                p(){(desc.get_clone())}
            }
            section(class="cantidad"){
                button(tabindex="-1",class=match disabled.get(){
                    false => "button restar",
                    true => "button restar disabled",
                },on:click = move |x|{
                    let val = props.valuable.clone();
                    focus(x);
                    let pos = props.pos;
                    let venta = props.venta.clone();
                    spawn_local(async move{
                        let res = call("descontar_producto_de_venta", DecrementarProductoDeVenta{ index: venta.with(|v|v
                            .productos
                            .iter()
                            .enumerate()
                            .find(|v| v.1 == &val)
                            .unwrap()
                            .0), pos }).await;
                        let venta_aux = from_value::<VentaSH>(res).unwrap();
                        venta.set(Venta::from_shared(venta_aux));
                    })
                },disabled=disabled.get()){"-"}
                input(r#type="number",class="cantidad-producto",on:focus=move |_|{props.focus.set(true)},bind:value=cantidad,on:keyup=move |e:KeyboardEvent|{
                    let event: KeyboardEvent = e.clone().unchecked_into();
                    if event.key().eq("Enter"){
                        cambio.set(true);
                        cambio.set(false);
                    }
                })
                button(tabindex="-1",class="button sumar",on:focus=move |_|{props.focus.set(true)},on:click=move|_|{
                    let val = valuable.clone();
                    let pos = props.pos;
                    let venta = props.venta.clone();
                    spawn_local(async move{
                        let res=call("incrementar_producto_a_venta", IncrementarProductoAVenta { index: venta.with(|v|v.productos.iter().enumerate().find(|v| v.1==&val).unwrap().0), pos }).await;
                        let venta_aux=from_value::<VentaSH>(res).unwrap();
                        venta.set(Venta::from_shared(venta_aux));
                    })
                }){"+"}
            }
            section(class="monto"){
                p(){
                    (format!("{:.2}",valuable3.get_unit_price()))
                }
            }
            section(){
                p(){
                    (format!("{:.2}",valuable4.clone().get_total_price()))
                }
            }
            section(id="borrar"){
                button(tabindex="-1",class="button eliminar",on:click=move |_|{
                    let val = valuable2.clone();
                    let pos = props.pos;
                    let venta = props.venta.clone();
                    spawn_local(async move{
                        let res=call("eliminar_producto_de_venta", EliminarProductoDeVenta { index: venta
                            .with(|v|v
                            .productos
                            .iter()
                            .enumerate()
                            .find(|v| v.1 == &val)
                            .unwrap()
                            .0), pos }).await;
                        let venta_aux=from_value::<VentaSH>(res).unwrap();
                        venta.set(Venta::from_shared(venta_aux));
                    })
                },on:focus=move |_|{props.focus.set(true)}){"Borrar"}
            }
        }
    )
}
