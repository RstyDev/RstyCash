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
#[derive(Prop)]
pub struct StateProps {
    pub venta_a: Rc<Venta>,
    pub venta_b: Rc<Venta>,
    pub config: Rc<Config>,
    pub clientes: Rc<Vec<Cliente>>,
    pub pos: Rc<bool>,
}

#[component]
pub fn MainPage<G: Html>(cx: Scope, props: StateProps) -> View<G> {
    let venta_a = create_signal_from_rc(cx, props.venta_a);
    let venta_b = create_signal_from_rc(cx, props.venta_b);
    let config = create_signal_from_rc(cx, props.config);
    let pos = create_signal_from_rc(cx, props.pos);
    let clientes = create_signal_from_rc(cx, props.clientes);
    view!(cx,
      header(){
        section(id="header"){
          div(){
            form(autocomplete="off"){
              input(type="text",id="buscador",placeholder="Buscar producto.."){}
            }
          }
          div(){
            SelectClientes(clientes= clientes.get())
          }
        }
      }
      main(class="main-screen"){
        CuadroPrincipal(venta_a=venta_a.get(), venta_b=venta_b.get(), config=config.get(), pos=pos.get(),clientes=clientes.get())
        ResumenPago(venta=match pos.get().as_ref(){
            true => venta_a.get(),
            false => venta_b.get(),
        },
        config=config.get())
      }
    )
}
