use sycamore::{futures::spawn_local_scoped, prelude::*};
use crate::mods::{lib::{call, debug}, structs::{args::EmptyArgs, Windows}};

#[derive(Prop)]
pub struct MenuProps{
    pub logged: RcSignal<bool>
}

#[allow(non_snake_case)]
#[component]
pub fn Menu<G: Html>(cx:Scope, props: MenuProps)->View<G>{
    view!(cx,
        div(id="menu"){
            section(){
                button(){"Opciones"}
                ul(class="dropdown"){
                    li(){a(){"Agregar cliente"}}
                    li(){a(){"Agregar producto"}}
                    li(){a(){"Agregar proveedor"}}
                    li(){a(){"Agregar usuario"}}
                    li(){a(){"Configuraciones"}}
                    li(){a(on:click= move |_|{
                        let logged = props.logged.clone();
                        spawn_local_scoped(cx, async move{
                            call("cerrar_sesion", EmptyArgs{}).await;
                            debug(logged.get().as_ref(),25,"menu");
                            logged.set(false);
                        });
                    }){"Cerrar sesion"}}
                }
            }
            section(){
                button(){"Venta"}
                ul(class="dropdown"){
                    li(){a(){"Guardar venta"}}
                    li(){a(){"Ventas guardadas"}}
                }
            }
            section(){
                button(){"Caja"}
                ul(class="dropdown"){
                    li(){a(){"Cerrar caja"}}
                }
            }
        }
    )
}
// pub fn get_menu() -> Menu {
//     let cerrar_caja_menu = CustomMenuItem::new(String::from("cerrar caja"), "Cerrar caja");
//     let add_product_menu = CustomMenuItem::new(String::from("add product"), "Agregar producto");
//     let add_prov_menu = CustomMenuItem::new(String::from("add prov"), "Agregar proveedor");
//     let add_user_menu = CustomMenuItem::new(String::from("add user"), "Agregar usuario");
//     let add_cliente_menu = CustomMenuItem::new(String::from("add cliente"), "Agregar cliente");
//     let cerrar_sesion_menu =
//         CustomMenuItem::new(String::from("cerrar sesion"), "Cerrar sesión");
//     let edit_settings_menu =
//         CustomMenuItem::new(String::from("edit settings"), "Cambiar configuraciones");
//     let confirm_stash_menu =
//         CustomMenuItem::new(String::from("confirm stash"), "Guardar venta");
//     let open_stash_menu =
//         CustomMenuItem::new(String::from("open stash"), "Ver ventas guardadas");
//     let opciones = Submenu::new(
//         "Opciones",
//         Menu::new()
//             .add_item(add_cliente_menu)
//             .add_item(add_product_menu)
//             .add_item(add_prov_menu)
//             .add_item(add_user_menu)
//             .add_item(edit_settings_menu)
//             .add_item(cerrar_sesion_menu),
//     );
//     let venta = Submenu::new(
//         "Venta",
//         Menu::new()
//             .add_item(confirm_stash_menu)
//             .add_item(open_stash_menu),
//     );
//     let caja = Submenu::new("Caja", Menu::new().add_item(cerrar_caja_menu));
//     Menu::new()
//         .add_submenu(opciones)
//         .add_submenu(venta)
//         .add_submenu(caja)
// }