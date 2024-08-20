use crate::mods::{
    main_window::{
        cuadro_principal::CuadroPrincipal,
        pagos::Pagos,
        producto::Prod,
        resumen_pago::{ResumenPago, ResumenProps},
        select_clientes::*,
    },
    structs::{
        Cliente, Config, Formato, Mayusculas, MedioPago, Pago, Pesable, Presentacion, Producto,
        Proveedor, RelacionProdProv, Rubro, Valuable, Venta, Windows,
    },
    Login,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::{rc::Rc, sync::Arc};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::rt::Event;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[derive(Prop, Clone, Debug, PartialEq)]
pub struct StateProps {
    pub venta_a: RcSignal<Venta>,
    pub venta_b: RcSignal<Venta>,
    pub config: RcSignal<Config>,
    pub clientes: RcSignal<Vec<Cliente>>,
    pub pos: RcSignal<bool>,
}

#[component]
pub fn MainPage<G: Html>(cx: Scope, props: StateProps) -> View<G> {
    let clientes = props.clientes.clone();

    let venta_a = props.venta_a.clone();
    let venta_b = props.venta_b.clone();
    let config = props.config.clone();
    let pos = props.pos.clone();
    view!(cx,
      header(){
        section(id="header"){
          div(){
            form(autocomplete="off"){
              input(type="text",id="buscador",placeholder="Buscar producto.."){}
            }
          }
          div(){
            SelectClientes(clientes= props.clientes.get())
          }
        }
      }
      main(class="main-screen"){
        CuadroPrincipal(venta_a=props.venta_a.clone(), venta_b=props.venta_b.clone(), config=props.config.clone(), pos=props.pos,clientes=clientes)
        ResumenPago(venta=match pos.get().as_ref(){
            true => venta_a.clone(),
            false => venta_b.clone(),
        },
        config=config.clone())
      }
    )
}
