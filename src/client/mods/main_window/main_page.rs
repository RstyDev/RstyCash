use crate::client::mods::{
    main_window::{main_section::*, select_clientes::*},
    structs::{Buscando, Cliente, Config, Nav, Pos, Venta},
};

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

#[derive(Props, Clone, Debug, PartialEq)]
pub struct StateProps {
    pub venta_a: Signal<Venta>,
    pub venta_b: Signal<Venta>,
    pub config: Signal<Config>,
    pub clientes: Signal<Vec<Cliente>>,
    pub pos: Signal<Pos>,
}
#[allow(non_snake_case)]
#[component]
pub fn MainPage(props: StateProps) -> View {
    let search_aux = create_signal(false);
    let nav = create_signal(Nav::Esc);
    let search = create_signal(String::new());
    let focus = create_signal(true);

    let buscando_aux = create_signal(Buscando::False {
        pos: props.pos.clone(),
        other_sale: props.pos.with(|p| match p {
            Pos::A { .. } => props.venta_b.clone(),
            Pos::B { .. } => props.venta_a.clone(),
        }),
        focus: focus.clone(),
    });


    create_effect(move || {
        if !search.with(|s| s.eq(&search.get_clone())) {
            search.set(search.get_clone());
        }
        if search.with(|s| s.len()) > 0 {
            buscando_aux.set(Buscando::True {
                nav: nav.clone(),
                search: search.clone(),
                pos: props.pos.clone(),
                other_sale: props.pos.with(|p| match p {
                    Pos::A { .. } => props.venta_b.clone(),
                    Pos::B { .. } => props.venta_a.clone(),
                }),
                aux: search_aux.clone(),
                focus: focus.clone(),
            })
        } else {
            buscando_aux.set(Buscando::False {
                pos: props.pos.clone(),
                other_sale: props.pos.with(|p| match p {
                    Pos::A { .. } => props.venta_b.clone(),
                    Pos::B { .. } => props.venta_a.clone(),
                }),
                focus: focus.clone(),
            })
        }
    });

    create_memo(move || {
        if search_aux.get() {
            search.set("".to_string());
            search_aux.set(false);
        }
    });

    create_memo(move || {
        props.venta_a.track();
        props.venta_b.track();
        match props.pos.get_clone() {
            Pos::A {
                config, clientes, ..
            } => {
                //buscando_aux.set(Buscando::False { venta: venta_a4.clone(), config: config.clone(), pos: pos3.clone() });
                props.pos.set(Pos::A {
                    venta: props.venta_a.clone(),
                    config: config.clone(),
                    clientes: clientes.clone(),
                })
            }
            Pos::B {
                config, clientes, ..
            } => {
                //buscando_aux.set(Buscando::False { venta: venta_b3.clone(), config: config.clone(), pos: pos3.clone() });
                props.pos.set(Pos::B {
                    venta: props.venta_b.clone(),
                    config: config.clone(),
                    clientes: clientes.clone(),
                })
            }
        }
    });
    view!(
        header(){
        section(id="header"){
          article(){
            form(autocomplete="off"){
              input(r#type="text",id="buscador",placeholder="Buscar producto..",bind:value=search,on:keydown=move|e:KeyboardEvent|{
                let keyup_event: KeyboardEvent = e.clone().unchecked_into();
                let key = keyup_event.key();
                match key.as_ref(){
                  "ArrowDown"=>{
                    e.prevent_default();
                    nav.set(Nav::Down);
                  },
                  "ArrowUp"=>{
                    e.prevent_default();
                    nav.set(Nav::Up);
                  },
                  "Escape"=>{
                    e.prevent_default();
                    nav.set(Nav::Esc);
                    search.set("".to_string());
                  },
                  "Enter"=>{
                    e.prevent_default();
                    nav.set(Nav::Enter);
                  },
                  _=>(),
                }
              },on:focus= move |_|{
                if !focus.get(){
                    focus.set(true)
                }
              })
            }
          }
          article(class="ayb"){
            a(on:click=move|_|{
                props.pos.set(Pos::A {venta:props.venta_a.clone(),config:props.config.clone(),clientes:props.clientes.clone()})

            },id="v-a",class=format!("a-boton {}",props.pos.with(|p|match p{Pos::A { .. }=>"v-actual",Pos::B { .. }=>""}))){
                "Venta A"
            }
            a(on:click=move|_|{
                props.pos.set(Pos::B {venta:props.venta_b.clone(),config:props.config.clone(),clientes:props.clientes.clone()})
            },id="v-a",class=format!("a-boton {}",props.pos.with(|p|match p{Pos::A { .. }=>"",Pos::B { ..  }=>"v-actual"}))){
                "Venta B"
            }
        }
          article(){
            SelectClientes(pos = props.pos.clone())
          }
        }
      }
      main(class="main-screen"){
        (match buscando_aux.get_clone(){
            Buscando::False { pos, .. } => {
              match pos.get_clone(){
                  Pos::A { venta, config, .. } => view!{MainSection(venta=venta.clone(),buscando=buscando_aux.clone(),config=config.clone())},
                  Pos::B { venta, config, .. } => view!{MainSection(venta=venta.clone(),buscando=buscando_aux.clone(),config=config.clone())},
              }
            },
            Buscando::True { pos, .. } => {
              match pos.get_clone(){
                  Pos::A { venta, config, .. } => view!{MainSection(venta=venta.clone(),buscando=buscando_aux.clone(),config=config.clone())},
                  Pos::B { venta, config, .. } => view!{MainSection(venta=venta.clone(),buscando=buscando_aux.clone(),config=config.clone())},
              }
            },
        })
      }
    )
}
