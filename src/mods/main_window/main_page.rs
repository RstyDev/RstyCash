
use std::rc::Rc;

use crate::mods::{
    lib::debug, main_window::{
        busqueda::Busqueda, cuadro_principal::CuadroPrincipal, resumen_pago::ResumenPago,
        select_clientes::*,
    }, structs::{Buscando, Cliente, Config, Nav, Pos, Venta}
};
use serde_wasm_bindgen::from_value;
use sycamore::{prelude::*, web::html::header};
use wasm_bindgen::prelude::*;
use web_sys::{Event, KeyboardEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
#[derive(Prop, Clone, Debug, PartialEq)]
pub struct StateProps{
    pub venta_a: RcSignal<Venta>,
    pub venta_b: RcSignal<Venta>,
    pub config: RcSignal<Config>,
    pub clientes: RcSignal<Vec<Cliente>>,
    pub pos: RcSignal<Pos>,
}

#[component]
pub fn MainPage<G: Html>(cx: Scope, props: StateProps) -> View<G> {
    let clientes = props.clientes.clone();
    let clientes1 = props.clientes.clone();
    let clientes2 = props.clientes.clone();
    let clientes3=props.clientes.clone();
    let nav = create_rc_signal(Nav::Esc);
    let nav2 = nav.clone();
    let search = create_signal(cx,String::new());
    let rc_search = create_rc_signal_from_rc(search.get());
    let rc_search1 = rc_search.clone();
    let rc_search2 = rc_search.clone();
    let venta_a1 = props.venta_a.clone();
    let venta_a2 = props.venta_a.clone();
    let venta_b1 = props.venta_b.clone();
    let venta_a3 = props.venta_a.clone();
    let venta_b2 = props.venta_b.clone();
    let venta_a4 = props.venta_a.clone();
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
  create_effect(cx, move ||{
    rc_search.set(search.get().as_ref().to_owned());
  });
  // create_memo(cx,move ||{
  //   let rc_s = rc_search2.clone();
  //   let rc = rc_s.get().as_ref().clone();
  //   if rc.eq("")&&!rc.eq(search.get().as_ref().into()){
  //     search.set(rc);
  //   }
  // });
    let pos_selector = create_selector(cx, move || pos2.get().as_ref().clone());
    let buscando = create_selector(cx, move || {

        if rc_search1.get().len() > 0 {
            Buscando::True {
                nav: nav2.clone(),
                search: rc_search1.clone(),
                pos: pos9.clone(),
            }
        } else {
            Buscando::False {
                venta: match pos1.get().as_ref() {
                    Pos::A {
                        venta,
                        config: _,
                        clientes: _,
                    } => venta.clone(),
                    Pos::B {
                        venta,
                        config: _,
                        clientes: _,
                    } => venta.clone(),
                },
                config: config1.clone(),
                clientes: clientes1.clone(),
                pos: pos1.clone(),
            }
        }
    });
    create_memo(cx, move || debug(pos4.get(),109,"main page"));
    create_memo(cx, move || {
        venta_a4.track();
        venta_b3.track();
        match pos3.get().as_ref() {
            Pos::A {
                venta: _,
                config,
                clientes,
            } => pos3.set(Pos::A {
                venta: venta_a4.clone(),
                config: config.clone(),
                clientes: clientes.clone(),
            }),
            Pos::B {
                venta: _,
                config,
                clientes,
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
            },id="v-a",class=format!("a-boton {}",match pos7.clone().get().as_ref(){Pos::A { venta:_, config:_, clientes:_,  }=>"v-actual",Pos::B { venta:_, config:_, clientes:_,  }=>""})){
                "Venta A"
            }
            a(on:click=move|_|{
                pos6.set(Pos::B { venta: venta_b2.clone(), config: config2.clone(), clientes:clientes2.clone(),  })
            },id="v-a",class=format!("a-boton {}",match props.pos.get().as_ref(){Pos::A { venta:_, config:_, clientes:_,  }=>"",Pos::B { venta:_, config:_, clientes:_,  }=>"v-actual"})){
                "Venta B"
            }
        }
          article(){
            (match pos_selector.get().as_ref(){
                Pos::A { venta, config:_, clientes,  } => view!(cx,SelectClientes(clientes= clientes.clone(),venta= venta.clone())),
                Pos::B { venta, config:_, clientes,  } => view!(cx,SelectClientes(clientes= clientes.clone(),venta= venta.clone())),
            })
          }
        }
      }
      main(class="main-screen"){


        (match pos_selector.get().as_ref().clone(){
            Pos::A { venta, config, clientes:_ , } => view!(cx,
              (match buscando.get().as_ref().clone(){
                Buscando::True{search, nav, pos } => view!(cx,
                  Busqueda(search = search.clone(), nav = nav.clone(), pos = pos.clone())
                ),
                Buscando::False { venta,  config, clientes, pos } => view!(cx,
                  CuadroPrincipal(venta=venta.clone(), config=config.clone(), pos= pos.clone(),clientes=clientes.clone())
                ),
            })
              ResumenPago(venta=venta.clone(),
              config=config.clone())
            ),
            Pos::B { venta, config, clientes:_ , } => view!(cx,
              (match buscando.get().as_ref().clone(){
                Buscando::True{search, nav, pos } => view!(cx,
                  Busqueda(search = search.clone(), nav = nav.clone(),pos = pos.clone())
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
