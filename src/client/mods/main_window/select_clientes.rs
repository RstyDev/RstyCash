use crate::client::mods::{
    lib::call,
    structs::{args::SetCliente, Cliente, Pos, Venta},
};
use sycamore::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Props)]
pub struct ClientesProps {
    pos: Signal<Pos>,
}
#[allow(non_snake_case)]
#[component]
pub fn SelectClientes(props: ClientesProps) -> View {
    let clientes = props.pos.with(|p| match p {
        Pos::A { clientes, .. } => clientes.clone(),
        Pos::B { clientes, .. } => clientes.clone(),
    });
    let id_select = create_signal(String::new());
    create_memo(move || {
        let venta = props.pos.with(|p| match p {
            Pos::A { venta, .. } => venta.clone(),
            Pos::B { venta, .. } => venta.clone(),
        });
        id_select.set(venta.with(|v| match &v.cliente {
            Cliente::Final => String::from("Final"),
            Cliente::Regular(c) => c.nombre.to_owned(),
        }));
    });
    view!(
        select(id="cliente",tabindex="-1",bind:value=id_select,on:change=move |_|{
            let cliente = clientes.with(|c|c.iter().find(|&c|{
                match (c,id_select.get_clone().as_ref()){
                    (Cliente::Final,"Final")=>true,
                    (Cliente::Regular(cli),nombre)=>cli.nombre.eq(nombre),
                    (_,_)=>false,
                }
            }).unwrap().clone());
            let cli = cliente.clone();
            spawn_local(async move{call("set_cliente",SetCliente{ id: match cli{
                Cliente::Final => 0,
                Cliente::Regular(cli) => cli.dni,
            }, pos: props.pos.with(|p|p.is_a()) }).await;});
            match props.pos.get_clone(){
                Pos::A { venta, .. } => venta.set(Venta{ cliente, ..venta.get_clone()}),
                Pos::B { venta, .. } => venta.set(Venta{ cliente, ..venta.get_clone()}),
            }
        }){
            Keyed(
                list=clientes,
                view=move |x|{
                    let nombre=x.get_nombre();
                    let nombre1=x.get_nombre();
                    view!{
                        option(selected=id_select.with(|i|i.eq(nombre.as_str()))){
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
        }
    )
}
