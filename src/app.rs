use crate::mods::{
    main_window::main_page::{MainPage, StateProps},
    structs::{
        Caja, Cliente, Config, Proveedor, Rango, Rcs, SistemaSH, User, UserSHC, Valuable, Venta, Windows
    },
    Login, LoginAux,
};

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::sync::Arc;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use wasm_bindgen_futures::spawn_local;

use wasm_bindgen::prelude::*;
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Payload {
    message: Option<String>,
    pos: Option<bool>,
    val: Option<Valuable>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, args: JsValue) -> JsValue;

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
    datos: Rcs,
) {
    let args = to_value(&LoginAux {
        user: datos.user.get().as_ref().clone().to_shared_complete(),
    })
    .unwrap();

    let res: JsValue = invoke("try_login", args).await;
    let res = from_value::<SistemaSH>(res);

    match res {
        Ok(a) => {
            datos.user.set(User::from_shared_complete(UserSHC {
                id: a.user.id,
                nombre: a.user.nombre,
                pass: [0, 0, 0, 0, 0, 0, 0, 0],
                rango: a.user.rango,
            }));
            datos.caja.set(Caja::from(a.caja));
            datos.config.set(Config::from(a.configs));
            datos
                .venta_a
                .set(Venta::from_shared_complete(a.ventas[0].clone()));
            datos
                .venta_b
                .set(Venta::from_shared_complete(a.ventas[1].clone()));
            let mut clientes = a
                .clientes
                .iter()
                .cloned()
                .map(|cli| Cliente::Regular(cli))
                .collect::<Vec<Cliente>>();
            clientes.insert(0, Cliente::Final);
            log(format!("{:#?}", clientes).as_str());
            datos.clientes.set(clientes);
            datos.proveedores.set(
                a.proveedores
                    .iter()
                    .map(|p| Proveedor::from_shared_complete(p.clone()))
                    .collect::<Vec<Proveedor>>(),
            );
            datos.logged.set(true);
        }
        Err(e) => log(e.to_string().as_str()),
    }
}
#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let caja = create_rc_signal(Caja::default());
    let conf = create_signal(cx, Config::default());
    let venta_a = create_signal(cx, Venta::default());
    let venta_b = create_signal(cx, Venta::default());
    let proveedores: RcSignal<Vec<Proveedor>> = create_rc_signal(Vec::new());
    let clientes = create_signal(cx, vec![Cliente::Final]);
    let logged_state = create_rc_signal(false);
    let user = create_rc_signal(User {
        id: "".to_string(),
        nombre: "".to_string(),
        pass: 1,
        rango: Rango::Cajero,
    });
    let pos_signal = create_signal(cx, true);

    let rc_user = create_rc_signal_from_rc(user.get());
    let rc_user_2 = rc_user.clone();
    let rc_user_3 = rc_user.clone();
    let rc_user_4 = rc_user.clone();
    let window = create_signal(cx, Windows::Login(rc_user_2));
    let logged_state_2 = logged_state.clone();
    let logged_state_3 = logged_state.clone();
    
    let rc_caja = create_rc_signal_from_rc(caja.get());
    let rc_conf = create_rc_signal_from_rc(conf.get());
    let rc_a = create_rc_signal_from_rc(venta_a.get());
    let rc_b = create_rc_signal_from_rc(venta_b.get());
    let rc_provs = create_rc_signal_from_rc(proveedores.get());
    let rc_clientes = create_rc_signal_from_rc(clientes.get());
    let rc_clientes_2 = rc_clientes.clone();
    let rc_a_2 = rc_a.clone();
    let rc_b_2 = rc_b.clone();
    let datos = Rcs{ user: rc_user_4, caja, config: rc_conf, venta_a: rc_a_2, venta_b: rc_b_2, proveedores, clientes: rc_clientes_2, logged: logged_state_2 };
    let rc_conf = datos.config.clone();
    
    
    let rc_clientes_2 = rc_clientes.clone();
    let rc_user_2 = rc_user.clone();
    let rc_a_2 = rc_a.clone();
    let rc_b_2 = rc_b.clone();
    let rc_pos_signal = create_rc_signal_from_rc(pos_signal.get());
    let rc_a_3 = rc_a.clone();
    create_effect(cx, move || match logged_state.get().as_ref() {
        false => window.set(Windows::Login(rc_user_3.clone())),
        true => window.set(Windows::Main(StateProps{ venta_a: rc_a_3.clone(), venta_b: rc_b_2.clone(), config: rc_conf.clone(), clientes: rc_clientes_2.clone(), pos: rc_pos_signal.clone() })),
    });
    let rc_b_2 = rc_b.clone();
    let rc_clientes_2 = rc_clientes.clone();
    let datos_2 = datos.clone();
    let _res = create_memo(cx, move || {
        let datos_3 = datos_2.clone();
        log(format!("Desde memo {:#?}", datos_2.user.get()).as_str());
        spawn_local(async move {
            try_login(datos_3).await;
        });
        rc_user_2.set_rc(datos_2.user.get());
        rc_a_2.set_rc(datos_2.venta_a.get());
        rc_b_2.set_rc(datos_2.venta_b.get());
        rc_clientes_2.set_rc(datos_2.clientes.get());
    });

    let rc_user_3 = rc_user.clone();

    let rend = create_selector(cx, move || {
        rc_user_3.get();
        window.get().as_ref().clone()
    });

    
    let clientes_2 = rc_clientes.clone();
    create_effect(cx, move || {
        log(format!("aca los clientes de app {:#?}", clientes_2.get()).as_str());
    });
    view!(cx,
        div{
            (
                match rend.get().as_ref() {
                Windows::Main(state) => {
                    log(format!("Esta en window {:#?}", window.get().as_ref()).as_str());
                    view! {cx,
                    div(id="cuadro-venta"){
                        MainPage(venta_a=state.venta_a.clone(),venta_b=state.venta_b.clone(),config=state.config.clone(),pos=state.pos.clone(),clientes=state.clientes.clone())
                    }
                }}
                Windows::Login(user) => {
                    log(format!("Esta en window {:#?}",window.get().as_ref()).as_str());
                    log(format!("y logged: {}",logged_state_3.get()).as_str());
                    view! {cx,
                    // div(id="cuadro-venta"){
                    //     MainPage(venta_a=venta_a.get(),venta_b=venta_b.get(),config=signal2.get(),pos=pos_signal.get(),clientes=clientes_signal.get())
                    // }
                    Login(user=user.clone())
                }}
            })
        }
    )
}
