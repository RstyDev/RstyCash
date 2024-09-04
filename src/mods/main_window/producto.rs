use std::rc::Rc;

use crate::mods::{lib::{call, debug}, structs::{args::EliminarProductoDeVenta, Config, Valuable, Venta, VentaSHC}};
use serde_wasm_bindgen::from_value;
use sycamore::{
    prelude::{component, view, Html, Prop, RcSignal, Scope, View}, reactive::{create_signal, create_signal_from_rc}
};
use wasm_bindgen_futures::spawn_local;

#[derive(Prop)]
pub struct ProdProps {
    venta: RcSignal<Venta>,
    valuable: Valuable,
    conf: Rc<Config>,
    pos: bool
}



#[component]
pub fn Prod<G: Html>(cx: Scope, props: ProdProps) -> View<G> {
    let conf = create_signal_from_rc(cx, props.conf);
    let desc = create_signal(cx, props.valuable.get_desc(conf.get().formato_producto));
    let val = props.valuable.clone();
    let val2 = props.valuable.clone();
    let rc_venta = props.venta.clone();
    let cant = create_signal(
        cx,
        match props.valuable {
            Valuable::Prod((cant, _)) => cant as f32,
            Valuable::Pes((cant, _)) => cant,
            Valuable::Rub((cant, _)) => cant as f32,
        },
    );
    view!(cx,
        article(class="articulo"){
            section(class=format!("descripcion {}",conf.get().modo_mayus)){
                p(){(desc.get())}
            }
            section(class="cantidad"){
                button(class="button restar",disabled=match cant.get().as_ref(){0.0=>true,_=>false,}){"-"}
                input(type="text",class="cantidad-producto",value=cant.get())
                button(class="button sumar"){"+"}
            }
            section(class="monto"){
                p(){
                    (match &props.valuable{
                        Valuable::Prod((_,prod)) => format!("{:.2}",prod.precio_venta),
                        Valuable::Pes((_,pes)) => format!("{:.2}",pes.precio_peso),
                        Valuable::Rub((_,rub)) => format!("{:.2}",rub.monto.unwrap_or_default()),
                    })
                }
            }
            section(){
                p(){
                    (match &val{
                        Valuable::Prod((cant,prod)) => format!("{:.2}",prod.precio_venta * *cant as f32),
                        Valuable::Pes((cant,pes)) => format!("{:.2}",pes.precio_peso * *cant),
                        Valuable::Rub((cant,rub)) => format!("{:.2}",rub.monto.unwrap_or_default() * *cant as f32),
                    })
                }
            }
            section(id="borrar"){
                button(class="button eliminar",on:click=move |_|{
                    let val = val2.clone();
                    let pos = props.pos;
                    let rc_venta = rc_venta.clone();
                    debug(val2.clone(),70,"producto");
                    spawn_local(async move{
                        let res=call("eliminar_producto_de_venta", EliminarProductoDeVenta { code: match val{
                            Valuable::Prod(prod) => prod.1.codigos_de_barras[0].to_be_bytes(),
                            Valuable::Pes(pes) => pes.1.codigo.to_be_bytes(),
                            Valuable::Rub(rub) => rub.1.codigo.to_be_bytes(),
                        }, pos }).await; 
                        let venta=from_value::<VentaSHC>(res).unwrap();
                        debug(venta.clone(),79,"producto");
                        rc_venta.set(Venta::from_shared_complete(venta));
                    })
                }){"Borrar"}
            }
        }
    )
}
