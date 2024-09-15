use crate::mods::{
    lib::call,
    structs::{args::SetCliente, Cliente, Pos, Venta},
};
use sycamore::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Prop)]
pub struct ClientesProps {
    pos: RcSignal<Pos>,
}
#[component]
pub fn SelectClientes<G: Html>(cx: Scope, props: ClientesProps) -> View<G> {
    let pos = props.pos.clone();
    let pos1 = pos.clone();

    let clientes = create_signal_from_rc(
        cx,
        match pos.get().as_ref() {
            Pos::A { clientes, .. } => clientes.get(),
            Pos::B { clientes, .. } => clientes.get(),
        },
    );

    let id_select = create_signal(cx, String::new());
    create_memo(cx, move || {
        let venta = match pos1.get().as_ref() {
            Pos::A { venta, .. } => venta.clone(),
            Pos::B { venta, .. } => venta.clone(),
        };
        id_select.set(match &venta.get().cliente {
            Cliente::Final => String::from("Final"),
            Cliente::Regular(c) => c.nombre.to_owned(),
        });
    });
    view!(cx,select(id="cliente",bind:value=id_select,on:change=move |_|{
        let cliente = clientes.get().iter().find(|c|{
            match (c,id_select.get().as_ref().as_str()){
                (Cliente::Final,"Final")=>true,
                (Cliente::Regular(cli),nombre)=>cli.nombre.eq(nombre),
                (_,_)=>false,
            }
        }).unwrap().clone();
        let cliente1=cliente.clone();
        let pos = pos.clone();
        let pos1 = pos.clone();
        spawn_local(async move{call("set_cliente",SetCliente{ id: match cliente1{
            Cliente::Final => 0,
            Cliente::Regular(cli) => cli.dni,
        }, pos: match pos.get().as_ref(){
            Pos::A { .. } => true,
            Pos::B { .. } => false,
        } }).await;});
        match pos1.get().as_ref(){
            Pos::A { venta, .. } => venta.set(Venta{ cliente: cliente, ..venta.get().as_ref().clone()}),
            Pos::B { venta, .. } => venta.set(Venta{ cliente: cliente, ..venta.get().as_ref().clone()}),
        }
    }){
        Keyed(
            iterable=clientes,
            view=move |cx,x|{
                let nombre=x.get_nombre();
                let nombre1=x.get_nombre();
                view!{cx,
                    option(selected=nombre.eq(id_select.get().as_ref())){
                        (nombre1)
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
    })
}
