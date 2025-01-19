use crate::client::mods::add_valuable::add_product::AddProduct;
use sycamore::{prelude::*,component};

#[allow(non_snake_case)]
#[component]
pub fn AddValuable() -> View {
    let selected = create_signal(String::from("Producto"));
    view! {
        div(r#class="space-between"){
            select(bind:value=selected){
                option(value="Producto"){"Producto"}
                option(value="Pesable"){"Pesable"}
                option(value="Rubro"){"Rubro"}
            }
            section(){
                button(){(format!("Agregar {}",selected.get_clone()))}
                button(){"Cancelar"}
            }
        }
        (match selected.get_clone().as_str(){
            "Producto" => view! {AddProduct()},
            _=>view! {},
            // "Pesable" => view!(cx, option(value="Pesable"){"Pesable"}),
            // "Rubro" => view!(cx, option(value="Rubro"){"Rubro"}),
        })
    }
}
