use crate::client::{menu::Menu, mods::{
    lib::{call, debug},
    main_window::main_page::{MainPage, StateProps},
    structs::{
        Caja, Cliente, Config, Pos, Proveedor, Rango, Rcs, SistemaSH, User, UserSHC, Valuable,
        Venta, Windows,
    },
    Login, LoginAux,
}};

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use std::sync::Arc;
use sycamore::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Payload {
    message: Option<String>,
    pos: Option<bool>,
    val: Option<Valuable>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User2 {
    pub id: Arc<str>,
    pub nombre: Arc<str>,
    pub pass: i64,
    pub rango: Rango,
}

async fn try_login(datos: Rcs) {
    let res = from_value::<SistemaSH>(
        call(
            "try_login",
            LoginAux {
                user: datos.user.get().as_ref().to_shared_complete(),
            },
        )
        .await,
    );
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
    datos.clientes.set(a.clientes.clone());
            datos.proveedores.set(
                a.proveedores
                .iter()
                    .map(|p| Proveedor::from_shared_complete(p.clone()))
                    .collect::<Vec<Proveedor>>(),
            );
            datos.logged.set(true);
        }
        Err(e) => debug(&e, 79, "app"),
    }
}
#[allow(non_snake_case)]
#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let rc_caja = create_rc_signal(Caja::default());
    let rc_conf = create_rc_signal(Config::default());
    let rc_a = create_rc_signal(Venta::default());
    let rc_b = create_rc_signal(Venta::default());
    let rc_provs: RcSignal<Vec<Proveedor>> = create_rc_signal(Vec::new());
    let rc_clientes = create_rc_signal(vec![Cliente::Final]);
    let rc_user = create_rc_signal(User {
        id: "".to_string(),
        nombre: "".to_string(),
        pass: 1,
        rango: Rango::Cajero,
    });
    let rc_logged = create_rc_signal(false);
    let rc_logged4 = rc_logged.clone();
    let window = create_signal(cx,Windows::Login(rc_user.clone()));
    let rc_a1 = rc_a.clone();
    let rc_conf1 = rc_conf.clone();
    let rc_clientes1 = rc_clientes.clone();
    let rc_pos = create_rc_signal(Pos::A {
        venta: rc_a1,
        config: rc_conf1,
        clientes: rc_clientes1,
    });
    let rc_conf1 = rc_conf.clone();
    let (rc_a1, rc_a2) = (rc_a.clone(), rc_a.clone());
    let (rc_b1, rc_b2) = (rc_b.clone(), rc_b.clone());
    let (rc_clientes1, rc_clientes2, rc_clientes3) = (
        rc_clientes.clone(),
        rc_clientes.clone(),
        rc_clientes.clone(),
    );
    let (rc_user1, rc_user2, rc_user3) = (rc_user.clone(), rc_user.clone(), rc_user.clone());
    let (rc_logged1, rc_logged2, rc_logged3) =
        (rc_logged.clone(), rc_logged.clone(), rc_logged.clone());
    
    let rend = create_selector(cx, move || window.get().as_ref().clone());
    let datos = Rcs {
        user: rc_user1,
        caja: rc_caja,
        config: rc_conf1,
        venta_a: rc_a1,
        venta_b: rc_b1,
        proveedores: rc_provs,
        clientes: rc_clientes1,
        logged: rc_logged1,
    };
    #[cfg(feature="ssr")]
    debug(&"Desde SSR",119,"app");

    create_memo(cx, move || match rc_logged2.get().as_ref() {
        false => window.set(Windows::Login(rc_user2.clone())),
        true => window.set(Windows::Main(StateProps {
            venta_a: rc_a2.clone(),
            venta_b: rc_b2.clone(),
            config: rc_conf.clone(),
            clientes: rc_clientes2.clone(),
            pos: rc_pos.clone(),
        })),
    });
    let datos_2 = datos.clone();
    let _res = create_memo(cx, move || {
        let datos_3 = datos_2.clone();
        let rc_logged = rc_logged3.clone();
        spawn_local(async move {
            if !rc_logged.get().as_ref() {
                try_login(datos_3).await;
            }
        });
        rc_user3.set_rc_silent(datos_2.user.get());
        rc_a.set_rc_silent(datos_2.venta_a.get());
        rc_b.set_rc_silent(datos_2.venta_b.get());
        rc_clientes3.set_rc_silent(datos_2.clientes.get());
    });
    view!(cx,
        Menu(logged=rc_logged4.clone())
        div{
            (
                match rend.get().as_ref() {
                Windows::Main(state) => {
                    view! {cx,
                    div(){
                        MainPage(venta_a=state.venta_a.clone(),venta_b=state.venta_b.clone(),config=state.config.clone(),pos=state.pos.clone(),clientes=state.clientes.clone())
                    }
                }}
                Windows::Login(user) => {
                    view! {cx,
                    Login(user=user.clone())
                }}
            })
        }
    )
}
