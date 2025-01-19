use serde::{Deserialize, Serialize};
use sycamore::{
    prelude::*,
    reactive::{create_signal, Signal},
    web::GlobalProps,
    Props,
};
use web_sys::SubmitEvent;
use crate::client::mods::lib::debug;
use crate::client::mods::structs::{get_hash, Rango, UserSHC};

use super::structs::User;

#[derive(Props)]
pub struct LoginProps {
    pub user: Signal<User>,
}
#[derive(Serialize, Deserialize)]
pub struct LoginAux {
    pub(crate) user: UserSHC,
}
#[allow(non_snake_case)]
#[component]
pub fn Login(props: LoginProps) -> View {
    let pass = create_signal(String::new());
    let user = create_signal(String::new());
    view! {
        form(id="form-login",on:submit=move|ev:SubmitEvent|{
            ev.prevent_default();
            //ev.stop_propagation();

            let id = user.get_clone();
            let pass = get_hash(&pass.get_clone());
            let print = format!("id: {}, pass: {}", id, pass);
            println!("{}",print);
            debug(&print, 34, "Login");
            debug(&user.get_clone(),32,"Login");
            props.user.set(User {
                id,
                nombre: String::new(),
                pass,
                rango: Rango::Cajero,
            });
        }){
            input(r#type="text",placeholder="Usuario",
            bind:value=user)
            input(r#type="password",placeholder="Contraseña",bind:value=pass)
            input(r#type="submit",value="Ingresar")
        }
    }
}
