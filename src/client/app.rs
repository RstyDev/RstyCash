use crate::client::{
    menu::Menu,
    mods::{
        lib::{call, debug},
        main_window::main_page::{MainPage, StateProps},
        structs::{
            Caja, Cliente, Config, Pos, Proveedor, Rango, Rcs, SistemaSH, User, UserSH, Valuable,
            Venta, Windows,
        },
        Login, LoginAux,
    },
};

use crate::client::mods::add_valuable::AddValuable;
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
                user: datos.user.with(|u| u.to_shared_complete()),
            },
        )
        .await,
    );
    match res {
        Ok(a) => {
            datos.user.set(User::from_shared(UserSH {
                id: a.user.id,
                nombre: a.user.nombre,
                rango: a.user.rango,
            }));
            datos.caja.set(Caja::from(a.caja));
            datos.config.set(Config::from(a.configs));
            datos.venta_a.set(Venta::from_shared(a.ventas[0].clone()));
            datos.venta_b.set(Venta::from_shared(a.ventas[1].clone()));
            datos.clientes.set(a.clientes.clone());
            datos.proveedores.set(
                a.proveedores
                    .into_iter()
                    .map(|p| Proveedor::from_shared(p))
                    .collect::<Vec<Proveedor>>(),
            );
            datos.logged.set(true);
        }
        Err(e) => debug(&e, 79, "app"),
    }
}
#[allow(non_snake_case)]
#[component]
pub fn App() -> View {
    let caja = create_signal(Caja::default());
    let conf = create_signal(Config::default());
    let v_a = create_signal(Venta::default());
    let v_b = create_signal(Venta::default());
    let proveedores: Signal<Vec<Proveedor>> = create_signal(Vec::new());
    let clientes = create_signal(vec![Cliente::Final]);
    let user = create_signal(User {
        id: "".to_string(),
        nombre: "".to_string(),
        pass: 1,
        rango: Rango::Cajero,
    });
    let logged = create_signal(false);

    let window = create_signal(Windows::Login(user.clone()));

    let pos = create_signal(Pos::A {
        venta: v_a.clone(),
        config: conf.clone(),
        clientes: clientes.clone(),
    });

    let rend = create_selector(move || window.get_clone());
    let datos = Rcs {
        user: user.clone(),
        caja,
        config: conf.clone(),
        venta_a: v_a.clone(),
        venta_b: v_b.clone(),
        proveedores,
        clientes: clientes.clone(),
        logged: logged.clone(),
    };

    create_memo(move || match logged.get() {
        false => window.set(Windows::Login(user.clone())),
        true => window.set(Windows::Main(StateProps {
            venta_a: v_a.clone(),
            venta_b: v_b.clone(),
            config: conf,
            clientes: clientes.clone(),
            pos: pos.clone(),
        })),
    });
    create_memo(move || {
        let rc_logged = logged.clone();
        let value = datos.clone();
        spawn_local(async move {
            if !rc_logged.get() {
                try_login(value).await;
            }
        });
        user.set_silent(datos.user.get_clone());
        v_a.set_silent(datos.venta_a.get_clone());
        v_b.set_silent(datos.venta_b.get_clone());
        clientes.set_silent(datos.clientes.get_clone());
    });
    view!(
        Menu(logged=logged)
        div{
            (
                match rend.get_clone() {
                Windows::Main(state) => {
                    view! {
                    div(){
                        MainPage(venta_a=state.venta_a.clone(),venta_b=state.venta_b.clone(),config=state.config.clone(),pos=state.pos.clone(),clientes=state.clientes.clone())
                    }
                }}
                Windows::Login(user) => {
                    view! {
                    Login(user=user)
                    AddValuable()
                }}
            })
        }
    )
}
