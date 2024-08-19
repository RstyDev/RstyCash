use std::rc::Rc;

use serde::{Deserialize, Serialize};
use sycamore::{
    prelude::{
        component, create_rc_signal_from_rc, create_signal_from_rc, view, Html, Scope, Signal, View,
    },
    Prop,
};
use sycamore::{
    reactive::{create_signal, RcSignal},
    web::html::input,
};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::Event;

use crate::mods::structs::{get_hash, Rango};

use super::structs::{User, UserSHC};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[derive(Prop)]
pub struct LoginProps {
    pub user: RcSignal<User>,
}
#[derive(Serialize, Deserialize)]
pub struct LoginAux {
    pub(crate) user: UserSHC,
}
#[derive(Default, Clone)]
pub struct UserArg {
    pub id: RcSignal<String>,
    pub nombre: RcSignal<String>,
    pub pass: RcSignal<i64>,
    pub rango: RcSignal<Rango>,
}
impl UserArg {
    pub fn new(
        id: RcSignal<String>,
        nombre: RcSignal<String>,
        pass: RcSignal<i64>,
        rango: RcSignal<Rango>,
    ) -> UserArg {
        UserArg {
            id,
            nombre,
            pass,
            rango,
        }
    }
}
#[component]
pub fn Login<G: Html>(cx: Scope, props: LoginProps) -> View<G> {
    //let us = create_signal(cx, props.args);
    //let dato=Event::new("type").unwrap().target().unwrap();

    // let us = create_rc_signal_from_rc(props.user);
    let pass = create_signal(cx, String::from(""));
    let user = create_signal(cx, String::from(""));
    let input_ingresar: View<G> = input()
        .attr("type", "submit")
        .attr("value", "Ingresar")
        .on("click", move |e: Event| {
            e.prevent_default();
            let id = user.get().to_string();
            let pass = get_hash(pass.get().as_str());
            props.user.set(User {
                id,
                nombre: String::from("nada"),
                pass,
                rango: Rango::Cajero,
            });
        })
        .view(cx);

    view! {cx,
        form(id="form-login"){
            input(type="text",placeholder="Usuario",
            bind:value=user,
            on:input=move |_|{
                log(format!("{:#?}",user.get()).as_str());
            })
            input(type="password",placeholder="Contraseña",bind:value=pass)

            // input(type="submit", value="Ingresar",on:click=move |d:Event|{
            //     d.prevent_default();
            //     props.id.set(user.get().to_string());
            //     props.pass.set(get_hash(pass.get().as_str()));
            //     //props.args.set(User { id: user.get().as_ref().to_string(), pass: get_hash(pass.get().as_str()),..props.args.get().as_ref().clone() });

            // })

            (input_ingresar)
        }
    }
}
