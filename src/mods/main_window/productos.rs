use crate::mods::{
    main_window::producto::Prod,
    structs::{Config, Venta},
};
use std::rc::Rc;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct ProdsProps {
    venta: RcSignal<Venta>,
    config: Rc<Config>,
    pos: bool,
}
#[component]
pub fn Productos<G: Html>(cx: Scope, props: ProdsProps) -> View<G> {
    let venta = props.venta.clone();
    let venta1 = props.venta.clone();
    let prods = create_signal(cx, props.venta.get().productos.clone());
    let conf = create_signal_from_rc(cx, props.config);
    create_memo(cx, move||{
        prods.set(venta1.get().productos.clone());
    });

    view! {cx,
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
                iterable = prods,
                view = move |cx,x|{
                    let venta = venta.clone();
                    view!{cx,Prod(valuable = x, conf = conf.get(), pos = props.pos, venta = venta.clone())}
                },
                key = |x|{x.id()}
            )
        }
    }
}
