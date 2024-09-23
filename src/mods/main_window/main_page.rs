use crate::mods::{
    main_window::{main_section::*, select_clientes::*},
    structs::{Buscando, Cliente, Config, Nav, Pos, Venta},
};

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::Event;

#[derive(Prop, Clone, Debug, PartialEq)]
pub struct StateProps {
    pub venta_a: RcSignal<Venta>,
    pub venta_b: RcSignal<Venta>,
    pub config: RcSignal<Config>,
    pub clientes: RcSignal<Vec<Cliente>>,
    pub pos: RcSignal<Pos>,
}

#[component]
pub fn MainPage<G: Html>(cx: Scope, props: StateProps) -> View<G> {
    let search_aux = create_rc_signal(false);
    let aux = search_aux.clone();
    let clientes2 = props.clientes.clone();
    let clientes3 = props.clientes.clone();
    let nav = create_rc_signal(Nav::Esc);
    let nav2 = nav.clone();
    let search = create_signal(cx, String::new());
    let rc_search = create_rc_signal_from_rc(search.get());
    let rc_search1 = rc_search.clone();
    let venta_a1 = props.venta_a.clone();
    let venta_a2 = props.venta_a.clone();
    let venta_a3 = props.venta_a.clone();
    let venta_a4 = props.venta_a.clone();
    let venta_b1 = props.venta_b.clone();
    let venta_b2 = props.venta_b.clone();
    let venta_b3 = props.venta_b.clone();
    let venta_b4 = props.venta_b.clone();
    let config2 = props.config.clone();
    let config3 = props.config.clone();
    let focus = create_rc_signal(true);
    let foc = focus.clone();
    let pos1 = props.pos.clone();
    let pos3 = props.pos.clone();
    let pos4 = props.pos.clone();
    let pos5 = props.pos.clone();
    let pos6 = props.pos.clone();
    let pos7 = props.pos.clone();
    let pos9 = props.pos.clone();
    let buscando_aux = create_rc_signal(Buscando::False {
        pos: pos1.clone(),
        other_sale: match pos1.get().as_ref() {
            Pos::A { .. } => venta_b4.clone(),
            Pos::B { .. } => venta_a3.clone(),
        },
        focus: focus.clone(),
    });

    let aux_buscando2 = buscando_aux.clone();

    create_effect(cx, move || {
        if !rc_search.get().as_ref().eq(search.get().as_ref()) {
            rc_search.set(search.get().as_ref().to_owned());
        }
        if rc_search.get().len() > 0 {
            buscando_aux.set(Buscando::True {
                nav: nav2.clone(),
                search: rc_search1.clone(),
                pos: pos9.clone(),
                other_sale: match pos9.get().as_ref() {
                    Pos::A { .. } => venta_b1.clone(),
                    Pos::B { .. } => venta_a2.clone(),
                },
                aux: search_aux.clone(),
                focus: focus.clone(),
            })
        } else {
            buscando_aux.set(Buscando::False {
                pos: pos1.clone(),
                other_sale: match pos1.get().as_ref() {
                    Pos::A { .. } => venta_b1.clone(),
                    Pos::B { .. } => venta_a2.clone(),
                },
                focus: focus.clone(),
            })
        }
    });

    create_memo(cx, move || {
        if *aux.get() {
            search.set("".to_string());
            aux.set(false);
        }
    });

    create_memo(cx, move || {
        venta_a4.track();
        venta_b3.track();
        match pos3.get().as_ref() {
            Pos::A {
                config, clientes, ..
            } => {
                //buscando_aux.set(Buscando::False { venta: venta_a4.clone(), config: config.clone(), pos: pos3.clone() });
                pos3.set(Pos::A {
                    venta: venta_a4.clone(),
                    config: config.clone(),
                    clientes: clientes.clone(),
                })
            }
            Pos::B {
                config, clientes, ..
            } => {
                //buscando_aux.set(Buscando::False { venta: venta_b3.clone(), config: config.clone(), pos: pos3.clone() });
                pos3.set(Pos::B {
                    venta: venta_b3.clone(),
                    config: config.clone(),
                    clientes: clientes.clone(),
                })
            }
        }
    });
    view!(cx,
        header(){
        section(id="header"){
          article(){
            form(autocomplete="off"){
              input(type="text",id="buscador",placeholder="Buscar producto..",bind:value=search,on:keydown=move|e:Event|{
                let keyup_event: web_sys::KeyboardEvent = e.clone().unchecked_into();
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
                if !*foc.get(){
                    foc.set(true)
                }
              })
            }
          }
          article(class="ayb"){
            a(on:click=move|_|{
                pos5.set(Pos::A {venta:venta_a1.clone(),config:config3.clone(),clientes:clientes3.clone()})

            },id="v-a",class=format!("a-boton {}",match pos7.clone().get().as_ref(){Pos::A { .. }=>"v-actual",Pos::B { .. }=>""})){
                "Venta A"
            }
            a(on:click=move|_|{
                pos6.set(Pos::B {venta:venta_b2.clone(),config:config2.clone(),clientes:clientes2.clone()})
            },id="v-a",class=format!("a-boton {}",match props.pos.get().as_ref(){Pos::A { .. }=>"",Pos::B { ..  }=>"v-actual"})){
                "Venta B"
            }
        }
          article(){
            SelectClientes(pos = pos4.clone())
          }
        }
      }
      main(class="main-screen"){
        (match aux_buscando2.get().as_ref(){
            Buscando::False { pos, focus, .. } => {
              let buscando=aux_buscando2.clone();
              let foc = focus.clone();
              match pos.get().as_ref(){
                  Pos::A { venta, config, .. } => view!{cx,MainSection(venta=venta.clone(),buscando=buscando.clone(),config=config.clone())},
                  Pos::B { venta, config, .. } => view!{cx,MainSection(venta=venta.clone(),buscando=buscando.clone(),config=config.clone())},
              }
            },
            Buscando::True { pos, focus, .. } => {
              let buscando = aux_buscando2.clone();
              let foc = focus.clone();
              match pos.get().as_ref(){
                  Pos::A { venta, config, .. } => view!{cx,MainSection(venta=venta.clone(),buscando=buscando.clone(),config=config.clone())},
                  Pos::B { venta, config, .. } => view!{cx,MainSection(venta=venta.clone(),buscando=buscando.clone(),config=config.clone())},
              }
            },
        })
        // (match pos_selector.get().as_ref().clone(){
        //     Pos::A { venta, config, .. } => view!{cx,MainSection(venta=venta.clone(),buscando=buscando.unwrap().clone(),config=config.clone())},
        //     Pos::B { venta, config, .. } => view!(cx,MainSection(venta=venta.clone(),buscando=buscando.unwrap().clone(),config=config.clone())),
        // })

      }
    )
}
