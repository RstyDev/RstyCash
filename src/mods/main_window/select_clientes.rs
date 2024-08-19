use crate::mods::{
    main_window::{cuadro_venta::*, main_page::*},
    structs::{Cliente, Config, Venta},
};
use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[derive(Prop)]
pub struct ClientesProps {
    clientes: Rc<Vec<Cliente>>,
}
#[component]
pub fn SelectClientes<G: Html>(cx: Scope, props: ClientesProps) -> View<G> {
    let clientes = create_signal_from_rc(cx, props.clientes);
    let actual: &Signal<Cliente> = create_signal(cx, clientes.get()[0].clone());
    let view: View<G> = view!(cx,select(id="cliente",value=match actual.get().as_ref(){
        Cliente::Final => "Final",
        Cliente::Regular(c) => c.nombre.as_ref(),
    }){
        Keyed(
            iterable=clientes,
            view=|cx,x|{
                view!{cx,
                    option(){
                        (match &x{
                            Cliente::Final => "Final".to_string(),
                            Cliente::Regular(c) => c.nombre.to_owned(),
                        })
                    }
                }
            },
            key=|x|{
                match x{
                    Cliente::Final => 0,
                    Cliente::Regular(c) => c.dni,
                }
            }
        )
    });
    log(format!("Aca estan los clientes   {:#?}", clientes.get()).as_str());

    view!(cx, (view))
}
/*<select id="cliente" value={selectValue(client)} disabled={disabledCli} onChange={(e)=>{select(e)}}>
    <option value='0' defaultValue="selected" >Consumidor Final</option>
    {clientes}
</select>); */
