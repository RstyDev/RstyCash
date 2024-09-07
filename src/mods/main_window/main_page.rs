use crate::mods::{
    lib::debug,
    main_window::{
        busqueda::Busqueda, cuadro_principal::CuadroPrincipal, resumen_pago::ResumenPago,
        select_clientes::*,
    },
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
    let clientes = props.clientes.clone();
    let clientes1 = props.clientes.clone();
    let clientes2 = props.clientes.clone();
    let clientes3 = props.clientes.clone();
    let nav = create_rc_signal(Nav::Esc);
    let nav2 = nav.clone();
    let nav3 = nav.clone();
    let search = create_signal(cx, String::new());
    let rc_search = create_rc_signal_from_rc(search.get());
    let rc_search1 = rc_search.clone();
    let rc_search2 = rc_search.clone();
    let venta_a1 = props.venta_a.clone();
    let venta_a2 = props.venta_a.clone();
    let venta_a3 = props.venta_a.clone();
    let venta_a4 = props.venta_a.clone();
    let venta_b1 = props.venta_b.clone();
    let venta_b2 = props.venta_b.clone();
    let venta_b3 = props.venta_b.clone();
    let config = props.config.clone();
    let config1 = props.config.clone();
    let config2 = props.config.clone();
    let config3 = props.config.clone();
    let pos1 = props.pos.clone();
    let pos2 = props.pos.clone();
    let pos3 = props.pos.clone();
    let pos4 = props.pos.clone();
    let pos5 = props.pos.clone();
    let pos6 = props.pos.clone();
    let pos7 = props.pos.clone();
    let pos8 = props.pos.clone();
    let pos9 = props.pos.clone();
    create_effect(cx, move || {
        if !rc_search.get().as_ref().eq(search.get().as_ref()) {
            rc_search.set(search.get().as_ref().to_owned());
        }
    });
    create_memo(cx, move || {
        if *aux.get() {
            search.set("".to_string());
            aux.set(false);
        }
        debug(aux.get(), 70, "main de rc_search");
        // if rc.as_str() == ""{
        //   search.set_rc(rc)
        // }
    });
    let pos_selector = create_selector(cx, move || pos2.get().as_ref().clone());
    let buscando = create_selector(cx, move || {
        if rc_search1.get().len() > 0 {
            Buscando::True {
                nav: nav2.clone(),
                search: rc_search1.clone(),
                pos: pos9.clone(),
                aux: search_aux.clone(),
            }
        } else {
            Buscando::False {
                venta: match pos1.get().as_ref() {
                    Pos::A {
                        venta,
                        ..
                    } => venta.clone(),
                    Pos::B {
                        venta,
                        ..
                    } => venta.clone(),
                },
                config: config1.clone(),
                clientes: clientes1.clone(),
                pos: pos1.clone(),
            }
        }
    });

    create_memo(cx, move || {
        venta_a4.track();
        venta_b3.track();
        match pos3.get().as_ref() {
            Pos::A {
                config,
                clientes,
                ..
            } => pos3.set(Pos::A {
                venta: venta_a4.clone(),
                config: config.clone(),
                clientes: clientes.clone(),
            }),
            Pos::B {
                config,
                clientes,
                ..
            } => pos3.set(Pos::B {
                venta: venta_b3.clone(),
                config: config.clone(),
                clientes: clientes.clone(),
            }),
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
                    //debug("down",137)
                  },
                  "ArrowUp"=>{
                    e.prevent_default();
                    nav.set(Nav::Up);
                    //debug("up",142)
                  },
                  "Escape"=>{
                    e.prevent_default();
                    nav.set(Nav::Esc);
                    search.set("".to_string());
                    //debug("esc",151)
                  },
                  "Enter"=>{
                    e.prevent_default();

                    nav.set(Nav::Enter);
                    //search.set("".to_string());
                    //debug("enter",156)
                  },
                  _=>(),
                }
              }){}
            }
          }
          article(class="ayb"){
            a(on:click=move|_|{
                pos5.set(Pos::A { venta: venta_a1.clone(), config: config3.clone(), clientes:clientes3.clone(),  })
            },id="v-a",class=format!("a-boton {}",match pos7.clone().get().as_ref(){Pos::A { .. }=>"v-actual",Pos::B { .. }=>""})){
                "Venta A"
            }
            a(on:click=move|_|{
                pos6.set(Pos::B { venta: venta_b2.clone(), config: config2.clone(), clientes:clientes2.clone(),  })
            },id="v-a",class=format!("a-boton {}",match props.pos.get().as_ref(){Pos::A { .. }=>"",Pos::B { ..  }=>"v-actual"})){
                "Venta B"
            }
        }
          article(){
            (match pos_selector.get().as_ref(){
                Pos::A { venta, clientes, ..  } => view!(cx,SelectClientes(clientes= clientes.clone(),venta= venta.clone())),
                Pos::B { venta, clientes, ..  } => view!(cx,SelectClientes(clientes= clientes.clone(),venta= venta.clone())),
            })
          }
        }
      }
      main(class="main-screen"){


        (match pos_selector.get().as_ref().clone(){
            Pos::A { venta, config, .. } => view!(cx,
              (match buscando.get().as_ref().clone(){
                Buscando::True{search,nav,pos, aux } => view!(cx,
                  Busqueda(search = search.clone(), nav = nav.clone(), pos = pos.clone(), search_aux = aux.clone())
                ),
                Buscando::False { venta,  config, clientes, pos } => view!(cx,
                  CuadroPrincipal(venta=venta.clone(), config=config.clone(), pos= pos.clone(),clientes=clientes.clone())
                ),
            })
              ResumenPago(venta=venta.clone(),
              config=config.clone())
            ),
            Pos::B { venta, config, .. } => view!(cx,
              (match buscando.get().as_ref().clone(){
                Buscando::True{search, nav, pos, aux } => view!(cx,
                  Busqueda(search = search.clone(), nav = nav.clone(),pos = pos.clone(), search_aux = aux.clone())
                ),
                Buscando::False { venta, config, clientes, pos } => view!(cx,
                  CuadroPrincipal(venta=venta.clone(), config=config.clone(), pos= pos.clone(),clientes=clientes.clone())
                ),
            })
              ResumenPago(venta=venta.clone(),
              config=config.clone())
            ),
        })

      }
    )
}
