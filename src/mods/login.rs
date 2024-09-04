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

#[derive(Prop)]
pub struct LoginProps {
    pub user: RcSignal<User>,
}
#[derive(Serialize, Deserialize)]
pub struct LoginAux {
    pub(crate) user: UserSHC,
}

#[component]
pub fn Login<G: Html>(cx: Scope, props: LoginProps) -> View<G> {
    let pass = create_signal(cx, String::new());
    let user = create_signal(cx, String::new());
    let input_ingresar: View<G> = input()
        .attr("type", "submit")
        .attr("value", "Ingresar")
        .on("click", move |e: Event| {
            e.prevent_default();
            let id = user.get().to_string();
            let pass = get_hash(pass.get().as_str());
            props.user.set(User {
                id,
                nombre: String::new(),
                pass,
                rango: Rango::Cajero,
            });
        })
        .view(cx);

    view! {cx,
        form(id="form-login"){
            input(type="text",placeholder="Usuario",
            bind:value=user)
            input(type="password",placeholder="Contraseña",bind:value=pass)
            (input_ingresar)
        }
    }
}
