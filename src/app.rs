use crate::mods::{
    main_window::main_page::MainPage,
    structs::{
        Caja, Cli, Cliente, Config, Cuenta, Formato, Mayusculas, MedioPago, Pago, Pesable,
        Presentacion, Producto, Proveedor, Rango, RelacionProdProv, Rubro, SistemaSH, User, 
        UserSHC, Valuable, Venta, Windows,
    },
    Login, LoginAux,
};
use web_sys::{window, Window};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::sync::Arc;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str) ->JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct User2 {
    pub id: Arc<str>,
    pub nombre: Arc<str>,
    pub pass: i64,
    pub rango: Rango,
}

async fn try_login(
    datos: (RcSignal<User>, RcSignal<Caja>, RcSignal<Config>, RcSignal<Venta>, RcSignal<Venta>, RcSignal<Vec<Proveedor>>, RcSignal<Vec<Cliente>>),
) {
    let args = to_value(&LoginAux {
        user: datos.0.get().as_ref().clone().to_shared_complete(),
    })
    .unwrap();

    let res: JsValue = invoke("try_login", args).await;
    let res = from_value::<SistemaSH>(res);

    match res {
        Ok(a) => {
            datos.0.set(User::from_shared_complete(UserSHC {
                id: a.user.id,
                nombre: a.user.nombre,
                pass: [0, 0, 0, 0, 0, 0, 0, 0],
                rango: a.user.rango,
            }));
            datos.1.set(Caja::from(a.caja));
            datos.2.set(Config::from(a.configs));
            datos
                .3
                .set(Venta::from_shared_complete(a.ventas[0].clone()));
            datos
                .4
                .set(Venta::from_shared_complete(a.ventas[1].clone()));
            datos.5.set(
                a.proveedores
                    .iter()
                    .map(|p| Proveedor::from_shared_complete(p.clone()))
                    .collect::<Vec<Proveedor>>(),
            );
            let mut clientes=vec![Cliente::Final];
            clientes.append(&mut a.clientes.iter().cloned().map(|c|Cliente::Regular(c)).collect::<Vec<Cliente>>());
            datos.6.set(clientes);
        }
        Err(e) => log(e.to_string().as_str()),
    }
}
#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let window = create_signal(cx, Windows::Login);
    let caja = create_rc_signal(Caja::default());
    let conf = create_rc_signal(Config::default());
    let venta_a = create_rc_signal(Venta::default());
    let venta_b = create_rc_signal(Venta::default());
    let proveedores: RcSignal<Vec<Proveedor>> = create_rc_signal(Vec::new());
    let clientes = create_rc_signal(vec![Cliente::Final]);
    let logged_state = create_signal(cx, false);
    
    create_effect(cx,move ||{
        if !logged_state.get().as_ref(){
            spawn_local_scoped(cx, async{
                invoke("open_login",JsValue::null()).await;
            })
        }
    });
    
    let pos_signal = create_signal(cx, true);
    let user = create_rc_signal(User::default());
    let datos = (
        user.clone(),
        caja.clone(),
        conf.clone(),
        venta_a.clone(),
        venta_b.clone(),
        proveedores.clone(),
        clientes.clone(),
    );
    let res=create_memo(cx, move || {
        let datos = datos.clone();
        log(format!("{:#?}",datos.0.get()).as_str());
        spawn_local_scoped(cx, async move { try_login(datos).await });
        log("ACA");
    });

    
    let clientes_signal = create_signal(
        cx,
        vec![
            Cliente::Final,
            Cliente::Regular(Cli::new(
                23,
                String::from("Lucas"),
                32,
                true,
                Utc::now().naive_local(),
                Cuenta::Auth(123.4),
            )),
        ],
    );
    
    

    match window.get().as_ref() {
        Windows::Main => view! {cx,
            div(id="cuadro-venta"){
                MainPage(venta_a=venta_a.get(),venta_b=venta_b.get(),config=conf.get(),pos=pos_signal.get(),clientes=clientes.get())
            }
        },
        Windows::Login => view! {cx,
            // div(id="cuadro-venta"){
            //     MainPage(venta_a=venta_a.get(),venta_b=venta_b.get(),config=signal2.get(),pos=pos_signal.get(),clientes=clientes_signal.get())
            // }
            Login(user=user)
        },
    }

}
