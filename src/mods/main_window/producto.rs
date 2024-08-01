use std::rc::Rc;

use crate::mods::structs::{Config, Valuable};
use sycamore::{
    prelude::{component, view, Html, Prop, Scope, Signal, View},
    reactive::{create_signal, create_signal_from_rc, ReadSignal},
};

#[derive(Prop)]
pub struct ProdProps {
    valuable: Valuable,
    conf: Rc<Config>,
}

#[component]
pub fn Prod<G: Html>(cx: Scope, props: ProdProps) -> View<G> {
    let conf = create_signal_from_rc(cx, props.conf);
    let desc = create_signal(cx, props.valuable.get_desc(conf.get().formato_producto));
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
                        Valuable::Prod((cant,prod)) => prod.precio_venta.to_string(),
                        Valuable::Pes((cant,pes)) => pes.precio_peso.to_string(),
                        Valuable::Rub((cant,rub)) => "".to_string(),
                    })
                }
            }
            section(){
                p(){

                }
            }
            section(id="borrar"){
                button(class="button eliminar"){"Borrar"}
            }
        }
    )
}
