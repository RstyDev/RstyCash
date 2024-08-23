use crate::mods::{
    main_window::{cuadro_venta::*, main_page::*},
    structs::{Cliente, Config, Venta},
};
use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[derive(Prop)]
pub struct ClientesProps {
    clientes: RcSignal<Vec<Cliente>>,
    venta: RcSignal<Venta>,
}
#[component]
pub fn SelectClientes<G: Html>(cx: Scope, props: ClientesProps) -> View<G> {
    let clientes = create_signal_from_rc(cx, props.clientes.get());
    let venta = props.venta.clone();

    let id_select = create_signal(cx, String::new());
    let actual: &Signal<Cliente> = create_signal(cx, clientes.get()[0].clone());
    let view: View<G> = view!(cx,select(id="cliente",bind:value=id_select,on:change=move |_|{
        let cliente = match id_select.get().as_ref().as_str(){
            "Final"=>Cliente::Final,
            nombre=>{
                let mut cliente=Cliente::Final;
                for cli in clientes.get().as_ref(){
                    match cli{
                        Cliente::Final => (),
                        Cliente::Regular(a) => if a.nombre.eq(nombre){
                            cliente = Cliente::Regular(a.clone())
                        },
                    }
                }
                cliente
            }
        };
        venta.set(Venta{ cliente: cliente, ..venta.get().as_ref().clone()})

    },value=match actual.get().as_ref(){
        Cliente::Final => "Final",
        Cliente::Regular(c) => c.nombre.as_ref(),
    }){
        Keyed(
            iterable=clientes,
            view=|cx,x|{
                view!{cx,
                    option(){
                        (match &x{
                            Cliente::Final => "Final".to_string(),
                            Cliente::Regular(c) => c.nombre.to_owned(),
                        })
                    }
                }
            },
            key=|x|{
                match x{
                    Cliente::Final => 0,
                    Cliente::Regular(c) => c.dni,
                }
            }
        )
    });
    view!(cx, (view))
}
