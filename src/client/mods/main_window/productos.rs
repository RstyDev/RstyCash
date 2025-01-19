use crate::client::mods::{
    main_window::producto::Prod,
    structs::{Config, Valuable, Venta},
};
use std::rc::Rc;
use sycamore::prelude::*;
use crate::client::mods::lib::debug;

#[derive(Props)]
pub struct ProdsProps {
    venta: Signal<Venta>,
    config: Signal<Config>,
    pos: bool,
    focus: Signal<bool>,
}
#[allow(non_snake_case)]
#[component]
pub fn Productos(props: ProdsProps) -> View {
    debug(&props.venta.get_clone(),18,"productos");
    let venta = props.venta.get_clone();
    let prods = venta.productos.clone();

    //let prods = create_memo(move || venta.map(|v|v.productos.clone()));
    //let prods = create_signal(props.venta.get_clone().productos.clone());
    let conf = props.config.clone();
    // create_memo(move || {
    //     prods.set(Vec::new());
    //     prods.set(props.venta.get_clone().productos.clone());
    // });

    view! {
        section(id="productos"){
            article(class="articulo"){
                section(class="descripcion"){
                    p{"DESCRIPCION"}
                }
                section(class="cantidad"){
                    p{"CANTIDAD"}
                }
                section(class="monto"){
                    p{"UNIDAD"}
                }
                section(){
                    p{"TOTAL PARCIAL"}
                }
            }
            Keyed(
                list = prods.clone(),
                view = move |x|{
                    let venta = props.venta.clone();
                    let focus = props.focus.clone();
                    view!{Prod(valuable = x.clone(), conf = conf.clone(), pos = props.pos, venta = venta, focus=focus)}
                },
                key = |x|{x.id()}
            )
        }
    }
}
