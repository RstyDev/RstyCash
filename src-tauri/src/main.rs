// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mods;
use commands::*;
use mods::{
    cmd::*, db::db, Caja, Cli, Cliente, Config, MedioPago, Pago, Producto, Proveedor, Rango,
    Result as Res, Rubro, Sistema, SistemaSH, User, UserSHC, Valuable as V, ValuableSH, Venta, VentaSHC,
};
use std::sync::Arc;
use tauri::{
    async_runtime::{block_on, Mutex},
    Manager, State, Window,
};

// #[tauri::command]
// fn agregar_cliente(
//     sistema: State<Mutex<Sistema>>,
//     window: Window,
//     nombre: &str,
//     dni: &str,
//     limite: Option<&str>,
// ) -> Res<Cli> {
//     agregar_cliente_2(sistema, window, nombre, dni, limite)
// }
#[tauri::command]
fn agregar_pago(
    window: Window,
    sistema: State<Mutex<Sistema>>,
    pago: Pago,
    pos: bool,
) -> Res<Vec<Pago>> {
    Ok(agregar_pago_2(window, sistema, pago, pos)?)
}
// #[tauri::command]
// fn agregar_pesable(
//     window: Window,
//     sistema: State<Mutex<Sistema>>,
//     precio_peso: &str,
//     codigo: &str,
//     costo_kilo: &str,
//     porcentaje: &str,
//     descripcion: &str,
// ) -> Res<String> {
//     agregar_pesable_2(
//         window,
//         sistema,
//         precio_peso,
//         codigo,
//         costo_kilo,
//         porcentaje,
//         descripcion,
//     )
// }
#[tauri::command]
async fn agregar_producto<'a>(
    window: Window,
    sistema: State<'a, Mutex<Sistema>>,
    producto: Producto,
) -> Res<String> {
    Ok(agregar_producto_2(window, sistema, producto)?)
}
#[tauri::command]
fn agregar_producto_a_venta(
    sistema: State<Mutex<Sistema>>,
    window: Window,
    prod: ValuableSH,
    pos: bool,
) -> Res<Venta> {
    Ok(agregar_producto_a_venta_2(sistema, window, prod, pos)?)
}
#[tauri::command]
fn agregar_proveedor(
    window: Window,
    sistema: State<Mutex<Sistema>>,
    proveedor: Proveedor,
) -> Res<()> {
    Ok(agregar_proveedor_2(window, sistema, proveedor)?)
}
#[tauri::command]
fn agregar_rubro(window: Window, sistema: State<Mutex<Sistema>>, rubro: Rubro) -> Res<String> {
    Ok(agregar_rubro_2(window, sistema, rubro)?)
}
#[tauri::command]
fn agregar_rub_o_pes_a_venta(
    sistema: State<Mutex<Sistema>>,
    window: Window,
    val: ValuableSH,
    pos: bool,
) -> Res<()> {
    Ok(agregar_rub_o_pes_a_venta_2(sistema, window, val, pos)?)
}
#[tauri::command]
fn agregar_usuario(window: Window, sistema: State<Mutex<Sistema>>, user: User) -> Res<String> {
    Ok(agregar_usuario_2(window, sistema, user)?)
}
#[tauri::command]
async fn cerrar_sesion<'a>(
    sistema: State<'a, Mutex<Sistema>>,
    handle: tauri::AppHandle,
) -> Res<()> {
    Ok(cerrar_sesion_2(sistema, handle).await?)
}
#[tauri::command]
fn cancelar_venta(sistema: State<Mutex<Sistema>>, pos: bool) -> Res<()> {
    Ok(cancelar_venta_2(sistema, pos)?)
}
#[tauri::command]
fn cerrar_caja(sistema: State<Mutex<Sistema>>, window: Window, monto_actual: f32) -> Res<()> {
    Ok(cerrar_caja_2(sistema, window, monto_actual)?)
}

#[tauri::command]
fn close_window(window: Window) -> Res<()> {
    Ok(close_window_2(window)?)
}
#[tauri::command]
fn descontar_producto_de_venta(
    sistema: State<Mutex<Sistema>>,
    index: usize,
    pos: bool,
) -> Res<Venta> {
    Ok(descontar_producto_de_venta_2(sistema, index, pos)?)
}
#[tauri::command]
fn editar_producto(sistema: State<Mutex<Sistema>>, prod: V) -> Res<()> {
    Ok(editar_producto_2(sistema, prod)?)
}
#[tauri::command]
fn eliminar_pago(sistema: State<Mutex<Sistema>>, pos: bool, id: i32) -> Res<Vec<Pago>> {
    Ok(eliminar_pago_2(sistema, pos, id)?)
}
#[tauri::command]
fn eliminar_producto(sistema: State<Mutex<Sistema>>, prod: V) -> Res<()> {
    Ok(eliminar_producto_2(sistema, prod)?)
}
#[tauri::command]
fn eliminar_producto_de_venta(
    sistema: State<Mutex<Sistema>>,
    window: Window,
    code: [u8;8],
    pos: bool,
) -> Res<VentaSHC> {
    let code = i64::from_be_bytes(code);
    Ok(eliminar_producto_de_venta_2(sistema, window, code, pos)?.to_shared_complete())
}
#[tauri::command]
fn eliminar_usuario(sistema: State<Mutex<Sistema>>, user: User) -> Res<()> {
    Ok(eliminar_usuario_2(sistema, user)?)
}
#[tauri::command]
fn get_caja(sistema: State<Mutex<Sistema>>) -> Res<Caja> {
    Ok(get_caja_2(sistema)?)
}
#[tauri::command]
fn get_clientes(sistema: State<Mutex<Sistema>>) -> Res<Vec<Cliente>> {
    Ok(get_clientes_2(sistema)?)
}
#[tauri::command]
fn get_configs(sistema: State<Mutex<Sistema>>) -> Res<Config> {
    Ok(get_configs_2(sistema)?)
}
#[tauri::command]
fn get_descripciones(prods: Vec<V>, conf: Config) -> Vec<(String, Option<f32>)> {
    get_descripciones_2(prods, conf)
}
#[tauri::command]
fn get_descripcion_valuable(prod: V, conf: Config) -> String {
    get_descripcion_valuable_2(prod, conf)
}
#[tauri::command]
fn get_deuda(sistema: State<Mutex<Sistema>>, cliente: Cli) -> Res<f32> {
    Ok(get_deuda_2(sistema, cliente)?)
}
#[tauri::command]
fn get_deuda_detalle(sistema: State<Mutex<Sistema>>, cliente: Cli) -> Res<Vec<Venta>> {
    Ok(get_deuda_detalle_2(sistema, cliente)?)
}
#[tauri::command]
fn get_filtrado(
    sistema: State<Mutex<Sistema>>,
    filtro: &str,
    tipo_filtro: &str,
) -> Res<Vec<String>> {
    Ok(get_filtrado_2(sistema, filtro, tipo_filtro)?)
}
#[tauri::command]
fn get_log_state(sistema: State<Mutex<Sistema>>) -> Res<bool> {
    Ok(get_log_state_2(sistema)?)
}
#[tauri::command]
fn get_medios_pago(sistema: State<Mutex<Sistema>>) -> Res<Vec<MedioPago>> {
    Ok(get_medios_pago_2(sistema)?)
}
#[tauri::command]
fn get_productos_filtrado(sistema: State<Mutex<Sistema>>, filtro: &str) -> Res<Vec<ValuableSH>> {
    Ok(get_productos_filtrado_2(sistema, filtro)?
        .iter()
        .map(|v| v.to_shared())
        .collect::<Vec<ValuableSH>>())
}
#[tauri::command]
fn get_proveedores(sistema: State<'_, Mutex<Sistema>>) -> Res<Vec<String>> {
    Ok(get_proveedores_2(sistema)?)
}
#[tauri::command]
fn get_rango(sistema: State<Mutex<Sistema>>) -> Res<Rango> {
    Ok(get_rango_2(sistema)?)
}
#[tauri::command]
fn get_stash(sistema: State<Mutex<Sistema>>) -> Res<Vec<Venta>> {
    Ok(get_stash_2(sistema)?)
}
#[tauri::command]
fn get_user(sistema: State<Mutex<Sistema>>) -> Res<User> {
    Ok(get_user_2(sistema)?)
}
#[tauri::command]
fn get_venta_actual(sistema: State<Mutex<Sistema>>, window: Window, pos: bool) -> Res<Venta> {
    Ok(get_venta_actual_2(sistema, window, pos)?)
}
#[tauri::command]
fn hacer_egreso(sistema: State<Mutex<Sistema>>, monto: f32, descripcion: Option<&str>) -> Res<()> {
    Ok(hacer_egreso_2(sistema, monto, descripcion)?)
}
#[tauri::command]
fn hacer_ingreso(sistema: State<Mutex<Sistema>>, monto: f32, descripcion: Option<&str>) -> Res<()> {
    Ok(hacer_ingreso_2(sistema, monto, descripcion)?)
}
#[tauri::command]
fn incrementar_producto_a_venta(
    sistema: State<Mutex<Sistema>>,
    index: usize,
    pos: bool,
) -> Res<Venta> {
    Ok(incrementar_producto_a_venta_2(sistema, index, pos)?)
}
#[tauri::command]
async fn open_add_prov(handle: tauri::AppHandle) -> Res<()> {
    Ok(open_add_prov_2(handle).await?)
}
#[tauri::command]
async fn open_add_product(handle: tauri::AppHandle) -> Res<()> {
    Ok(open_add_product_2(handle).await?)
}
#[tauri::command]
async fn open_add_user(handle: tauri::AppHandle) -> Res<()> {
    Ok(open_add_user_2(handle).await?)
}
#[tauri::command]
async fn open_add_cliente(handle: tauri::AppHandle) -> Res<()> {
    Ok(open_add_cliente_2(handle).await?)
}
#[tauri::command]
async fn open_cancelar_venta(handle: tauri::AppHandle, act: bool) -> Res<()> {
    Ok(open_cancelar_venta_2(handle, act).await?)
}
#[tauri::command]
async fn open_cerrar_caja(handle: tauri::AppHandle) -> Res<()> {
    Ok(open_cerrar_caja_2(handle).await?)
}
#[tauri::command]
async fn open_confirm_stash(handle: tauri::AppHandle, act: bool) -> Res<()> {
    Ok(open_confirm_stash_2(handle, act).await?)
}
#[tauri::command]
async fn open_edit_settings(handle: tauri::AppHandle) -> Res<()> {
    Ok(open_edit_settings_2(handle).await?)
}
#[tauri::command]
async fn open_login(handle: tauri::AppHandle, window: Window) -> Res<()> {
    loop {
        if window
            .emit("main", Payload::new(Some(String::from("aca")), None, None))
            .is_ok()
        {
            break;
        }
    }
    Ok(open_login_2(handle).await?)
}
#[tauri::command]
async fn open_select_amount(handle: tauri::AppHandle, val: V, pos: bool) -> Res<()> {
    open_select_amount_2(handle, val, pos)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
async fn open_stash<'a>(
    handle: tauri::AppHandle,
    sistema: State<'a, Mutex<Sistema>>,
    pos: bool,
) -> Res<()> {
    open_stash_2(handle, sistema, pos)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
fn pagar_deuda_especifica(
    sistema: State<Mutex<Sistema>>,
    cliente: i32,
    venta: Venta,
) -> Res<Venta> {
    Ok(pagar_deuda_especifica_2(sistema, cliente, venta)?)
}
#[tauri::command]
fn pagar_deuda_general(sistema: State<Mutex<Sistema>>, cliente: i64, monto: f32) -> Res<f32> {
    Ok(pagar_deuda_general_2(sistema, cliente, monto)?)
}
#[tauri::command]
fn set_cantidad_producto_venta(
    sistema: State<Mutex<Sistema>>,
    index: usize,
    cantidad: &str,
    pos: bool,
) -> Res<Venta> {
    Ok(set_cantidad_producto_venta_2(
        sistema, index, cantidad, pos,
    )?)
}
#[tauri::command]
fn set_cliente(sistema: State<Mutex<Sistema>>, id: i32, pos: bool) -> Res<Venta> {
    Ok(set_cliente_2(sistema, id, pos)?)
}
#[tauri::command]
fn set_configs(window: Window, sistema: State<Mutex<Sistema>>, configs: Config) -> Res<()> {
    Ok(set_configs_2(window, sistema, configs)?)
}
#[tauri::command]
fn stash_n_close(window: Window, sistema: State<Mutex<Sistema>>, pos: bool) -> Res<()> {
    Ok(stash_n_close_2(window, sistema, pos)?)
}
#[tauri::command]
fn try_login(sistema: State<Mutex<Sistema>>, window: Window, user: UserSHC) -> Res<SistemaSH> {
    Ok(try_login_2(
        sistema,
        window,
        User::from_shared_complete(user),
    )?)
}
#[tauri::command]
fn unstash_sale(sistema: State<Mutex<Sistema>>, window: Window, pos: bool, index: &str) -> Res<()> {
    Ok(unstash_sale_2(sistema, window, pos, index)?)
}

fn main() {
    let menu = get_menu();
    let db = Arc::from(block_on(db()).unwrap());
    let app = tauri::Builder::default()
        .manage(Mutex::new(Sistema::new(db).unwrap()))
        .invoke_handler(tauri::generate_handler![
            //agregar_cliente,
            agregar_pago,
            // agregar_pesable,
            agregar_producto,
            agregar_producto_a_venta,
            agregar_proveedor,
            agregar_rubro,
            agregar_rub_o_pes_a_venta,
            agregar_usuario,
            cancelar_venta,
            cerrar_caja,
            cerrar_sesion,
            //check_codes,
            close_window,
            descontar_producto_de_venta,
            editar_producto,
            eliminar_pago,
            eliminar_producto,
            eliminar_producto_de_venta,
            eliminar_usuario,
            get_caja,
            get_clientes,
            get_configs,
            get_descripciones,
            get_descripcion_valuable,
            get_deuda,
            get_deuda_detalle,
            get_filtrado,
            get_log_state,
            get_medios_pago,
            get_productos_filtrado,
            get_proveedores,
            get_rango,
            get_stash,
            get_user,
            get_venta_actual,
            hacer_egreso,
            hacer_ingreso,
            incrementar_producto_a_venta,
            open_add_prov,
            open_add_product,
            open_add_user,
            open_add_cliente,
            open_cancelar_venta,
            open_cerrar_caja,
            open_confirm_stash,
            open_edit_settings,
            open_login,
            open_select_amount,
            open_stash,
            pagar_deuda_especifica,
            pagar_deuda_general,
            try_login,
            set_cantidad_producto_venta,
            set_cliente,
            set_configs,
            stash_n_close,
            unstash_sale,
        ])
        .menu(menu)
        .build(tauri::generate_context!())
        .expect("error while building tauri application");
    let window = app.get_window("main").unwrap();

    let w2 = window.clone();
    let handle = app.handle();
    window.on_menu_event(move |event| {
        match event.menu_item_id() {
            "add product" => block_on(open_add_product(handle.clone())),
            "add prov" => block_on(open_add_prov(handle.clone())),
            "add user" => block_on(open_add_user(handle.clone())),
            "add cliente" => block_on(open_add_cliente(handle.clone())),
            "edit settings" => block_on(open_edit_settings(handle.clone())),
            "confirm stash" => {
                loop {
                    if w2
                        .emit(
                            "main",
                            Payload::new(Some(String::from("confirm stash")), None, None),
                        )
                        .is_ok()
                    {
                        break;
                    }
                }
                Ok(())
            }
            "cerrar sesion" => {
                loop {
                    if w2
                        .emit(
                            "main",
                            Payload::new(Some(String::from("cerrar sesion")), None, None),
                        )
                        .is_ok()
                    {
                        break;
                    }
                }
                Ok(())
            }

            "open stash" => {
                loop {
                    if w2
                        .emit(
                            "main",
                            Payload::new(Some(String::from("open stash")), None, None),
                        )
                        .is_ok()
                    {
                        break;
                    }
                }
                Ok(())
            }
            "cerrar caja" => block_on(open_cerrar_caja(handle.clone())),

            _ => Ok(()),
        }
        .unwrap();
    });
    app.run(|_, _| {})
}
