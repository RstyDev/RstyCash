use crate::client::mods::{lib::{call, debug},structs::{args::EmptyArgs, Proveedor, ProveedorSH}};
use serde_wasm_bindgen::from_value;
use sycamore::{{{prelude::*, futures::spawn_local_scoped}, web::{GlobalAttributes, GlobalProps}},component, view};
use web_sys::{MouseEvent,SubmitEvent,Event};
// fn agregar_codigo(codigos: &Signal<Vec<i64>>, code: i64){
//     codigos.get().insert(codigos.get().len() - 2, code);
// }

// fn agregar_producto(producto: ProductoSH){
//
// }
async fn get_provs() -> Vec<ProveedorSH> {
    let value = call("get_proveedores", EmptyArgs {}).await;
    let res = from_value::<Vec<ProveedorSH>>(value);
    res.unwrap()
}

#[allow(non_snake_case)]
#[component]
pub fn AddProduct() -> View {
    let codigos: Signal<Vec<i64>> = create_signal(vec![]);
    let proveedores: Signal<Vec<Proveedor>> = create_signal(Vec::new());
    let costo = create_signal(0.0);

    let prov_options = create_signal(Vec::new());
    spawn_local_scoped(async move {
        prov_options.set(
            get_provs()
                .await
                .into_iter()
                .map(|p| Proveedor::from_shared(p))
                .collect(),
        );
    });
    // create_effect(cx,|| {
    //     debug(prov_options.get(),31,"add_product");
    // });
    // let prov_options = create_resource(cx,get_provs());
    let prov_id = create_signal(String::new());
    let prov_cod = create_signal(String::new());
    let code = create_signal(String::new());
    let provs_left = create_selector(move || prov_options.with(|p| p.len()) > 0);
    let less_than_3 = create_selector(move || codigos.with(|c| c.len()) < 3);

    create_effect(move || {
        if prov_options.with(|p| p.len()) > 0 {
            prov_id.set(prov_options.with(|p| p[0].id.to_string()));
        }
    });
    // on_mount(cx,||async{
    //     prov_options.set(get_provs().await.into_iter().map(|p|Proveedor::from_shared(p)).collect());
    // });

    view! {
        form(id="agregar-producto-container"){
            section(class="frame left"){
                p(){"Producto"}
                article(class="form producto"){
                    section(){
                        label(r#for="costo"){"Precio de costo: "}
                        input(r#type="number", id="costo", placeholder="Precio de costo",bind:valueAsNumber=costo)
                    }
                    section(){
                        label(r#for="porcentaje"){"Porcentaje: "}
                        input(r#type="number", id="porcentaje", placeholder="Porcentaje")
                    }
                    section(){
                        label(r#for="precio"){"Precio de venta: "}
                        input(r#type="number", id="precio", placeholder="Precio de venta")
                    }
                    section(){
                        label(r#for="tipo"){"Tipo de producto: "}
                        input(r#type="text", id="tipo", placeholder="Tipo de producto")
                    }
                    section(){
                        label(r#for="marca"){"Marca: "}
                        input(r#type="text", id="marca", placeholder="Marca")
                    }
                    section(){
                        label(r#for="variedad"){"Variedad"}
                        input(r#type="text", id="variedad", placeholder="Variedad")
                    }
                    section(){
                        label(r#for="presentacion"){"Presentación: "}
                        input(r#type="number", id="presentacion", placeholder="Presentacion")
                        select(){
                            option(value="Gr"){"gr"}
                            option(value="Un"){"un"}
                            option(value="Lt"){"Lt"}
                            option(value="Ml"){"ml"}
                            option(value="CC"){"cm3"}
                            option(value="Kg"){"Kg"}
                        }
                    }
                }
            }
            section(class="frame right first"){
                p(){"Códigos de barras"}
                article(class="form codigos"){
                    Keyed(
                        list = codigos,
                        view= move |x| {
                            view!{
                                section(){
                                    input(r#type="number", value=x.to_string(), disabled=true)
                                    button(on:click= move |ev:MouseEvent|{
                                        ev.prevent_default();
                                        let mut codes = codigos.get_clone();
                                        codes.remove(codes.iter().enumerate().find(|y|*y.1 == x).unwrap().0);
                                        codigos.set(codes);
                                    }){"Borrar"}
                                }
                            }
                        },
                        key=|x|*x,
                    )
                    (match less_than_3.get(){
                        true=>{
                            view!{
                            form(id="cod_form",on:submit=move |ev:SubmitEvent|{
                                ev.prevent_default();
                                if code.with(|c|c.len())>0{
                                    let mut codes = codigos.get_clone();
                                    codes.push(code.with(|c|c.parse::<i64>().unwrap()));
                                    codigos.set(codes);
                                    code.set(String::new());
                                }
                            }){
                                input(r#type="number", bind:value=code, placeholder = "Código de barras")
                                button(){"Agregar"}
                            }
                        }},
                        false=>view!{},
                    })

                }
            }
            section(class="frame right second"){
                p(){"Proveedores"}
                article(class="form proveedores"){
                    Keyed(
                        list = proveedores,
                        view = move |prov| {
                            view! {
                                section(){
                                    input(r#type="text", value = prov.nombre, disabled = true)
                                    input(r#type="number", value = prov.contacto.map(|c|c.to_string()).unwrap_or_default(), disabled = true)
                                    button(on:click=move |ev:MouseEvent|{
                                        ev.prevent_default();
                                        let mut provs = proveedores.get_clone();
                                        let i = provs.iter().enumerate().find(|p|p.1.id == prov.id).unwrap().0;
                                        let current = provs.remove(i);
                                        let mut options = prov_options.get_clone();
                                        options.push(current);
                                        prov_options.set(options);
                                        proveedores.set(provs);

                                    }){"Borrar"}
                                }
                            }
                        },
                        key = |prov|prov.id,
                    )
                    (match provs_left.get(){
                        true=>{
                            view! {
                            form(id="prov_form",on:submit= move |ev:SubmitEvent|{
                                ev.prevent_default();
                                let id = prov_id.with(|i|i.parse::<i32>().unwrap());
                                let i = prov_options.with(|p|p.iter().enumerate().find(|p|p.1.id == id).unwrap().0);
                                let mut provs = prov_options.get_clone();
                                let prov = provs.remove(i);
                                let mut provs_aux = proveedores.get_clone();
                                provs_aux.push(Proveedor{ contacto: match prov_cod.with(|p|p.len()){
                                    0=>None,
                                    _=>Some(prov_cod.with(|c|c.parse::<i64>().unwrap())),
                                },..prov});
                                proveedores.set(provs_aux);
                                prov_options.set(provs);
                                prov_cod.set(String::new());

                            }){
                                select(bind:value = prov_id,on:change = move |ev:Event|{
                                    debug(&ev.as_string(), 127, "add_product");
                                    debug(&prov_id.get_clone(),128,"add_product");
                                    // prov_name.set(ev.as_string().unwrap());
                                }){
                                    Keyed(
                                        list = prov_options,
                                        view = |opt| view!{option(value=opt.id.to_string()){(opt.nombre)}},
                                        key = |opt| opt.id
                                    )
                                }
                                input(r#type="number", bind:value=prov_cod, placeholder="Código interno")
                                button(){"Agregar"}
                                // button(on:click = |e:Event|{
                                //     // e.prevent_default();
                                //     //     debug(&prov_id.get().len(),143,"add_producto");
                                //     //     let id = prov_id.get().parse::<i32>().unwrap();
                                //     //     let i = prov_options.get().as_ref().into_iter().enumerate().find(|p|p.1.id == id).unwrap().0;
                                //     //     let mut provs = prov_options.get().as_ref().clone();
                                //     //     let prov = provs.remove(i);
                                //     //     let mut provs_aux = proveedores.get().as_ref().clone();
                                //     //     provs_aux.push(Proveedor{ contacto: match prov_cod.get().as_ref().len(){
                                //     //         0=>None,
                                //     //         _=>Some(prov_cod.get().parse::<i64>().unwrap()),
                                //     //     },..prov});
                                //     //     proveedores.set(provs_aux);
                                //     //     prov_options.set(provs);
                                //
                                //         // spawn_local(async {
                                //         //     prov_options.set(get_provs().await.into_iter().map(|p|Proveedor::from_shared(p)).collect());
                                //         // });
                                //
                                // }){"Agregar"}
                            }
                        }},
                        false=>view! { },
                    })

                }
            }
        }
    }
}
