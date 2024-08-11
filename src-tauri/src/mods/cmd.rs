use super::Valuable as V;
use serde::Serialize;
const INDEX: &str = "index.html";
const DENEGADO: &str = "Permiso denegado";
#[derive(Clone, Serialize)]
pub struct Payload {
    message: Option<String>,
    pos: Option<bool>,
    val: Option<V>,
}
impl Payload {
    pub fn new(message: Option<String>, pos: Option<bool>, val: Option<V>) -> Payload {
        Payload { message, pos, val }
    }
}
pub mod commands {
    use crate::mods::{
        cmd::{Payload, DENEGADO, INDEX},
        get_hash,
        sistema::SistemaSH,
        AppError, Caja, Cli, Config, Pago, Pesable, Producto, Proveedor, Rango, Res, Rubro,
        Sistema, User, Valuable as V, Venta,
    };
    use std::sync::Arc;
    use tauri::async_runtime::{block_on, spawn, Mutex};
    use tauri::window::MenuHandle;
    use tauri::{
        AppHandle, CustomMenuItem, Manager, Menu, State, Submenu, Window, WindowBuilder, WindowUrl,
    };

    pub fn get_menu() -> Menu {
        let cerrar_caja_menu = CustomMenuItem::new(String::from("cerrar caja"), "Cerrar caja");
        let add_product_menu = CustomMenuItem::new(String::from("add product"), "Agregar producto");
        let add_prov_menu = CustomMenuItem::new(String::from("add prov"), "Agregar proveedor");
        let add_user_menu = CustomMenuItem::new(String::from("add user"), "Agregar usuario");
        let add_cliente_menu = CustomMenuItem::new(String::from("add cliente"), "Agregar cliente");
        let cerrar_sesion_menu =
            CustomMenuItem::new(String::from("cerrar sesion"), "Cerrar sesión");
        let edit_settings_menu =
            CustomMenuItem::new(String::from("edit settings"), "Cambiar configuraciones");
        let confirm_stash_menu =
            CustomMenuItem::new(String::from("confirm stash"), "Guardar venta");
        let open_stash_menu =
            CustomMenuItem::new(String::from("open stash"), "Ver ventas guardadas");
        let opciones = Submenu::new(
            "Opciones",
            Menu::new()
                .add_item(add_cliente_menu)
                .add_item(add_product_menu)
                .add_item(add_prov_menu)
                .add_item(add_user_menu)
                .add_item(edit_settings_menu)
                .add_item(cerrar_sesion_menu),
        );
        let venta = Submenu::new(
            "Venta",
            Menu::new()
                .add_item(confirm_stash_menu)
                .add_item(open_stash_menu),
        );
        let caja = Submenu::new("Caja", Menu::new().add_item(cerrar_caja_menu));
        Menu::new()
            .add_submenu(opciones)
            .add_submenu(venta)
            .add_submenu(caja)
    }
    fn set_menus(menu: MenuHandle, state: bool) -> Res<()> {
        menu.get_item("add product").set_enabled(state)?;
        menu.get_item("add prov").set_enabled(state)?;
        menu.get_item("add user").set_enabled(state)?;
        Ok(menu.get_item("edit settings").set_enabled(state)?)
    }
    pub fn agregar_cliente_2(
        sistema: State<Mutex<Sistema>>,
        window: Window,
        cliente: Cli,
    ) -> Res<Cli> {
        let sis = block_on(sistema.lock());
        sis.access();
        let cli = sis.agregar_cliente(cliente)?;
        loop {
            if window
                .emit(
                    "main",
                    Payload {
                        message: Some(String::from("dibujar venta")),
                        pos: None,
                        val: None,
                    },
                )
                .is_ok()
            {
                break;
            }
        }
        close_window_2(window)?;
        Ok(cli)
    }

    pub fn agregar_pago_2(
        window: Window,
        sistema: State<Mutex<Sistema>>,
        pago: Pago,
        pos: bool,
    ) -> Res<Vec<Pago>> {
        let mut sis = block_on(sistema.lock());
        sis.access();
        sis.agregar_pago(pago, pos)?;
        if sis.venta(pos).pagos().len() == 0 {
            loop {
                if window
                    .emit(
                        "main",
                        Payload {
                            message: Some(String::from("dibujar venta")),
                            pos: None,
                            val: None,
                        },
                    )
                    .is_ok()
                {
                    break;
                }
            }
        }
        Ok(sis.venta(pos).pagos())
    }
    pub fn agregar_pesable_2<'a>(
        window: Window,
        sistema: State<Mutex<Sistema>>,
        mut pesable: Pesable,
    ) -> Res<String> {
        let sis = block_on(sistema.lock());
        sis.access();
        match sis.arc_user().rango() {
            Rango::Admin => {
                let pesable = block_on(pesable.new_to_db(sis.db()))?;
                close_window_2(window)?;
                Ok(pesable)
            }
            Rango::Cajero => Err(AppError::IncorrectError(DENEGADO.to_string())),
        }
    }
    pub fn agregar_producto_2(
        window: Window,
        sistema: State<Mutex<Sistema>>,
        producto: Producto,
    ) -> Res<String> {
        let mut sis = block_on(sistema.lock());
        match sis.arc_user().rango() {
            Rango::Admin => {
                let prod = block_on(sis.agregar_producto(producto))?;
                close_window_2(window)?;
                Ok(format!("Agregado {prod:#?}"))
            }
            Rango::Cajero => Err(AppError::IncorrectError(DENEGADO.to_string())),
        }
    }
    pub fn agregar_producto_a_venta_2(
        sistema: State<Mutex<Sistema>>,
        window: Window,
        prod: V,
        pos: bool,
    ) -> Res<Venta> {
        let mut sis = block_on(sistema.lock());
        sis.access();
        match &prod {
            V::Prod(_) => {
                block_on(sis.agregar_producto_a_venta(prod, pos))?;
                loop {
                    if let Ok(_) = window
                        .menu_handle()
                        .get_item("confirm stash")
                        .set_enabled(true)
                    {
                        break;
                    }
                }
            }
            V::Pes(a) => {
                spawn(open_select_amount_2(
                    window.app_handle(),
                    V::Pes(a.clone()),
                    pos,
                ));
            }
            V::Rub(a) => {
                spawn(open_select_amount_2(
                    window.app_handle(),
                    V::Rub(a.clone()),
                    pos,
                ));
            }
        }
        Ok(sis.venta(pos))
    }

    pub fn agregar_proveedor_2(
        window: Window,
        sistema: State<Mutex<Sistema>>,
        proveedor: Proveedor,
    ) -> Res<()> {
        let mut sis = block_on(sistema.lock());
        match sis.arc_user().rango() {
            Rango::Admin => {
                sis.agregar_proveedor(proveedor)?;
                Ok(close_window_2(window)?)
            }
            Rango::Cajero => Err(AppError::IncorrectError(DENEGADO.to_string())),
        }
    }
    pub fn agregar_rubro_2(
        window: Window,
        sistema: State<Mutex<Sistema>>,
        rubro: Rubro,
    ) -> Res<String> {
        let sis = block_on(sistema.lock());
        match sis.arc_user().rango() {
            Rango::Admin => {
                let rubro = block_on(rubro.new_to_db(sis.db()))?;
                close_window_2(window)?;
                Ok(rubro.descripcion().to_string())
            }
            Rango::Cajero => Err(AppError::IncorrectError(DENEGADO.to_string())),
        }
    }
    pub fn agregar_rub_o_pes_a_venta_2(
        sistema: State<Mutex<Sistema>>,
        window: Window,
        val: V,
        pos: bool,
    ) -> Res<()> {
        let mut sis = block_on(sistema.lock());
        block_on(sis.agregar_producto_a_venta(val, pos))?;
        loop {
            if window
                .emit(
                    "main",
                    Payload {
                        message: Some(String::from("dibujar venta")),
                        pos: None,
                        val: None,
                    },
                )
                .is_ok()
            {
                break;
            }
        }
        Ok(close_window_2(window)?)
    }
    pub fn agregar_usuario_2(
        window: Window,
        sistema: State<Mutex<Sistema>>,
        user: User,
    ) -> Res<String> {
        let sis = block_on(sistema.lock());
        match sis.arc_user().rango() {
            Rango::Admin => {
                let res = sis.agregar_usuario(user)?;
                close_window_2(window)?;
                Ok(res)
            }
            Rango::Cajero => Err(AppError::IncorrectError(DENEGADO.to_string())),
        }
    }
    pub async fn cerrar_sesion_2<'a>(
        sistema: State<'a, Mutex<Sistema>>,
        handle: AppHandle,
    ) -> Res<()> {
        let mut sis = sistema.lock().await;
        match handle.get_window("login") {
            Some(window) => {
                loop {
                    if window.show().is_ok() {
                        break;
                    }
                }

                Ok(sis.cerrar_sesion())
            }
            None => {
                WindowBuilder::new(
                    &handle,
                    "login", /* the unique window label */
                    WindowUrl::App("/pages/login.html".parse().unwrap()),
                )
                .inner_size(400.0, 300.0)
                .resizable(false)
                .minimizable(false)
                .closable(false)
                .always_on_top(true)
                .decorations(false)
                .center()
                .menu(Menu::new())
                .build()?;
                Ok(sis.cerrar_sesion())
            }
        }
    }
    pub fn cancelar_venta_2(sistema: State<Mutex<Sistema>>, pos: bool) -> Res<()> {
        let mut sis = block_on(sistema.lock());
        sis.access();
        sis.cancelar_venta(pos)
    }
    pub fn cerrar_caja_2(
        sistema: State<Mutex<Sistema>>,
        window: Window,
        monto_actual: f32,
    ) -> Res<()> {
        let mut sis = block_on(sistema.lock());
        sis.access();
        sis.cerrar_caja(monto_actual)?;
        Ok(close_window_2(window)?)
    }
    pub fn close_window_2(window: Window) -> Res<()> {
        loop {
            if window.close().is_ok() {
                break;
            }
        }
        Ok(())
    }
    pub fn descontar_producto_de_venta_2(
        sistema: State<Mutex<Sistema>>,
        index: usize,
        pos: bool,
    ) -> Res<Venta> {
        let mut sis = block_on(sistema.lock());
        sis.access();
        let res = sis.descontar_producto_de_venta(index, pos)?;
        Ok(res)
    }
    pub fn editar_producto_2(sistema: State<Mutex<Sistema>>, prod: V) -> Res<()> {
        let sis = block_on(sistema.lock());
        sis.access();
        sis.editar_valuable(prod);
        Ok(())
    }
    pub fn eliminar_pago_2(sistema: State<Mutex<Sistema>>, pos: bool, id: i32) -> Res<Vec<Pago>> {
        let mut sis = block_on(sistema.lock());
        sis.access();
        sis.eliminar_pago(pos, id)
    }
    pub fn eliminar_producto_2(sistema: State<Mutex<Sistema>>, prod: V) -> Res<()> {
        let sis = block_on(sistema.lock());
        sis.access();
        sis.eliminar_valuable(prod);
        Ok(())
    }
    pub fn eliminar_producto_de_venta_2(
        sistema: State<Mutex<Sistema>>,
        window: Window,
        index: usize,
        pos: bool,
    ) -> Res<Venta> {
        let mut sis = block_on(sistema.lock());
        sis.access();
        let res = sis.eliminar_producto_de_venta(index, pos)?;
        loop {
            if window
                .menu_handle()
                .get_item("confirm stash")
                .set_enabled(false)
                .is_ok()
            {
                break;
            }
        }
        Ok(res)
    }
    pub fn eliminar_usuario_2(sistema: State<Mutex<Sistema>>, user: User) -> Res<()> {
        let res = block_on(sistema.lock());
        res.access();
        Ok(res.eliminar_usuario(user)?)
    }
    pub fn get_caja_2(sistema: State<Mutex<Sistema>>) -> Res<Caja> {
        let sis = block_on(sistema.lock());
        sis.access();
        Ok(sis.caja().clone())
    }
    pub fn get_clientes_2(sistema: State<Mutex<Sistema>>) -> Res<Vec<Cli>> {
        let sis = block_on(sistema.lock());
        sis.access();
        Ok(block_on(sis.get_clientes())?)
    }
    pub fn get_configs_2(sistema: State<Mutex<Sistema>>) -> Res<Config> {
        Ok(block_on(sistema.lock()).configs().clone())
    }
    pub fn get_descripciones_2(prods: Vec<V>, conf: Config) -> Vec<(String, Option<f32>)> {
        prods
            .iter()
            .map(|p| (p.descripcion(&conf), p.price(&conf.politica())))
            .collect::<Vec<(String, Option<f32>)>>()
    }
    pub fn get_descripcion_valuable_2(prod: V, conf: Config) -> String {
        prod.descripcion(&conf)
    }
    pub fn get_deuda_2(sistema: State<Mutex<Sistema>>, cliente: Cli) -> Res<f32> {
        let sis = block_on(sistema.lock());
        sis.access();
        sis.get_deuda(cliente)
    }
    pub fn get_deuda_detalle_2(sistema: State<Mutex<Sistema>>, cliente: Cli) -> Res<Vec<Venta>> {
        let sis = block_on(sistema.lock());
        sis.access();
        sis.get_deuda_detalle(cliente)
    }
    pub fn get_filtrado_2(
        sistema: State<Mutex<Sistema>>,
        filtro: &str,
        tipo_filtro: &str,
    ) -> Res<Vec<String>> {
        let sis = block_on(sistema.lock());
        sis.access();
        match tipo_filtro {
            "marca" => Ok(sis.filtrar_marca(filtro)?),
            "tipo_producto" => Ok(sis.filtrar_tipo_producto(filtro)?),
            _ => Err(AppError::IncorrectError(format!(
                "Parámetro incorrecto {tipo_filtro}"
            ))),
        }
    }
    pub fn get_log_state_2(sistema: State<Mutex<Sistema>>) -> Res<bool> {
        Ok(block_on(sistema.lock()).user().is_some())
    }
    pub fn get_medios_pago_2(sistema: State<Mutex<Sistema>>) -> Res<Vec<String>> {
        let sis = block_on(sistema.lock());
        sis.access();
        Ok(sis
            .configs()
            .medios_pago()
            .iter()
            .map(|m| m.to_string())
            .collect())
    }
    pub fn get_productos_filtrado_2(sistema: State<Mutex<Sistema>>, filtro: &str) -> Res<Vec<V>> {
        let sis = block_on(sistema.lock());
        sis.access();
        Ok(block_on(sis.val_filtrado(filtro, sis.db()))?)
    }
    pub fn get_proveedores_2(sistema: State<'_, Mutex<Sistema>>) -> Res<Vec<String>> {
        let sis = block_on(sistema.lock());
        sis.access();
        Ok(block_on(sis.proveedores())?
            .iter()
            .map(|x| x.to_string())
            .collect())
    }
    pub fn get_rango_2(sistema: State<Mutex<Sistema>>) -> Res<Rango> {
        Ok(block_on(sistema.lock()).arc_user().rango().clone())
    }
    pub fn get_stash_2(sistema: State<Mutex<Sistema>>) -> Res<Vec<Venta>> {
        let sis = block_on(sistema.lock());
        sis.access();
        Ok(sis.stash().clone())
    }
    pub fn get_user_2(sistema: State<Mutex<Sistema>>) -> Res<User> {
        Ok(block_on(sistema.lock()).arc_user().as_ref().clone())
    }
    pub fn get_venta_actual_2(
        sistema: State<Mutex<Sistema>>,
        window: Window,
        pos: bool,
    ) -> Res<Venta> {
        let sis = block_on(sistema.lock());
        sis.access();
        let venta = sis.venta(pos);
        if venta.productos().len() == 0 {
            loop {
                if window
                    .menu_handle()
                    .get_item("confirm stash")
                    .set_enabled(false)
                    .is_ok()
                {
                    break;
                }
            }
        } else {
            loop {
                if window
                    .menu_handle()
                    .get_item("confirm stash")
                    .set_enabled(true)
                    .is_ok()
                {
                    break;
                }
            }
        }
        println!("{:#?}", venta);
        Ok(venta)
    }
    pub fn hacer_egreso_2(
        sistema: State<Mutex<Sistema>>,
        monto: f32,
        descripcion: Option<&str>,
    ) -> Res<()> {
        let sis = block_on(sistema.lock());
        Ok(sis.hacer_egreso(monto, descripcion.map(|d| Arc::from(d)))?)
    }
    pub fn hacer_ingreso_2(
        sistema: State<Mutex<Sistema>>,
        monto: f32,
        descripcion: Option<&str>,
    ) -> Res<()> {
        let sis = block_on(sistema.lock());
        Ok(sis.hacer_ingreso(monto, descripcion.map(|d| Arc::from(d)))?)
    }
    pub fn incrementar_producto_a_venta_2(
        sistema: State<Mutex<Sistema>>,
        index: usize,
        pos: bool,
    ) -> Res<Venta> {
        let mut sis = block_on(sistema.lock());
        sis.access();
        let venta = sis.incrementar_producto_a_venta(index, pos)?;
        Ok(venta)
    }
    pub async fn open_add_prov_2(handle: AppHandle) -> Res<()> {
        match handle.get_window("add-prov") {
            Some(window) => Ok(window.show()?),
            None => {
                WindowBuilder::new(
                    &handle,
                    "add-prov", /* the unique window label */
                    WindowUrl::App(INDEX.parse().unwrap()),
                )
                .always_on_top(true)
                .center()
                .resizable(false)
                .minimizable(false)
                .inner_size(330.0, 210.0)
                .menu(Menu::new())
                .title("Agregar Proveedor")
                .build()?;
                Ok(())
            }
        }
    }
    pub async fn open_add_product_2(handle: AppHandle) -> Res<()> {
        match handle.get_window("add-prod") {
            Some(window) => Ok(window.show()?),
            None => {
                WindowBuilder::new(&handle, "add-prod", WindowUrl::App(INDEX.parse().unwrap()))
                    .always_on_top(true)
                    .center()
                    .resizable(false)
                    .minimizable(false)
                    .title("Seleccione una opción")
                    .inner_size(600.0, 380.0)
                    .menu(Menu::new())
                    .build()?;
                Ok(())
            }
        }
    }

    pub async fn open_add_user_2(handle: AppHandle) -> Res<()> {
        match handle.get_window("add-user") {
            Some(window) => Ok(window.show()?),
            None => {
                WindowBuilder::new(
                    &handle,
                    "add-user", /* the unique window label */
                    WindowUrl::App(INDEX.parse().unwrap()),
                )
                .always_on_top(true)
                .center()
                .resizable(false)
                .minimizable(false)
                .title("Agregar Usuario")
                .inner_size(430.0, 200.0)
                .menu(Menu::new())
                .build()?;
                Ok(())
            }
        }
    }
    pub async fn open_add_cliente_2(handle: AppHandle) -> Res<()> {
        match handle.get_window("add-cliente") {
            Some(window) => Ok(window.show()?),
            None => {
                WindowBuilder::new(
                    &handle,
                    "add-cliente",
                    WindowUrl::App(INDEX.parse().unwrap()),
                )
                .always_on_top(true)
                .center()
                .resizable(false)
                .minimizable(false)
                .title("Agregar Cliente")
                .inner_size(400.0, 230.0)
                .menu(Menu::new())
                .build()?;
                Ok(())
            }
        }
    }
    pub async fn open_cancelar_venta_2(handle: AppHandle, act: bool) -> Res<()> {
        //TODO!(Hay que ver si es necesario usar un mismo html o no asi evi el window.emit)
        match handle.get_window("confirm-cancel") {
            Some(window) => {
                window.show()?;
                window.emit(
                    "get-venta",
                    Payload {
                        message: Some(String::from("cancelar venta")),
                        pos: Some(act),
                        val: None,
                    },
                )?;
                Ok(())
            }
            None => {
                WindowBuilder::new(
                    &handle,
                    "confirm-cancel",
                    WindowUrl::App(INDEX.parse().unwrap()),
                )
                .always_on_top(true)
                .center()
                .resizable(false)
                .minimizable(false)
                .inner_size(400.0, 150.0)
                .menu(Menu::new())
                .title("Confirmar")
                .build()?;
                Ok(())
            }
        }
    }
    pub async fn open_cerrar_caja_2(handle: AppHandle) -> Res<()> {
        match handle.get_window("cerrar-caja") {
            Some(window) => Ok(window.show()?),
            None => {
                WindowBuilder::new(
                    &handle,
                    "cerrar-caja",
                    WindowUrl::App(INDEX.parse().unwrap()),
                )
                .always_on_top(true)
                .center()
                .resizable(false)
                .minimizable(false)
                .title("Cerrar Caja")
                .inner_size(640.0, 620.0)
                .menu(Menu::new())
                .build()?;
                Ok(())
            }
        }
    }
    pub async fn open_confirm_stash_2(handle: AppHandle, act: bool) -> Res<()> {
        //TODO!(Aca la otra parte que usa el confirm)
        match handle.get_window("confirm-stash") {
            Some(window) => {
                window.show()?;
                window.emit(
                    "get-venta",
                    Payload {
                        message: Some(String::from("stash")),
                        pos: Some(act),
                        val: None,
                    },
                )?;
                Ok(())
            }
            None => {
                let win = WindowBuilder::new(
                    &handle,
                    "confirm-stash", /* the unique window label */
                    WindowUrl::App(INDEX.parse().unwrap()),
                )
                .always_on_top(true)
                .center()
                .resizable(false)
                .minimizable(false)
                .inner_size(400.0, 150.0)
                .menu(Menu::new())
                .title("Confirmar Stash")
                .build()?;
                std::thread::sleep(std::time::Duration::from_millis(500));
                win.emit(
                    "get-venta",
                    Payload {
                        message: Some(String::from("stash")),
                        pos: Some(act),
                        val: None,
                    },
                )?;
                for _ in 0..7 {
                    std::thread::sleep(std::time::Duration::from_millis(175));
                    win.emit(
                        "get-venta",
                        Payload {
                            message: Some(String::from("stash")),
                            pos: Some(act),
                            val: None,
                        },
                    )?;
                }
                Ok(())
            }
        }
    }
    pub async fn open_edit_settings_2(handle: tauri::AppHandle) -> Res<()> {
        match handle.get_window("edit-settings") {
            Some(window) => Ok(window.show()?),
            None => {
                WindowBuilder::new(
                    &handle,
                    "edit-settings", /* the unique window label */
                    WindowUrl::App(INDEX.parse().unwrap()),
                )
                .always_on_top(true)
                .center()
                .resizable(false)
                .minimizable(false)
                .inner_size(500.0, 360.0)
                .menu(Menu::new())
                .title("Configuraciones")
                .build()?;
                Ok(())
            }
        }
    }
    pub async fn open_login_2(handle: tauri::AppHandle) -> Res<()> {
        handle.get_window("main").unwrap().minimize()?;
        match handle.get_window("login") {
            Some(window) => {
                window.show()?;
                Ok(window.set_focus()?)
            }
            None => {
                let window = WindowBuilder::new(
                    &handle,
                    "login", /* the unique window label */
                    WindowUrl::App(INDEX.parse().unwrap()),
                )
                .inner_size(400.0, 300.0)
                .resizable(false)
                .minimizable(false)
                .closable(false)
                .always_on_top(true)
                .decorations(false)
                .center()
                .title("Iniciar Sesión")
                .menu(Menu::new())
                .build()?;
                window.set_focus()?;
                Ok(())
            }
        }
    }
    pub async fn open_select_amount_2(handle: tauri::AppHandle, val: V, pos: bool) -> Res<()> {
        match handle.get_window("select-amount") {
            Some(window) => {
                window.show()?;
                std::thread::sleep(std::time::Duration::from_millis(400));
                let mut res = Err(AppError::IncorrectError(String::from(
                    "Error emitiendo mensaje",
                )));
                for _ in 0..8 {
                    std::thread::sleep(std::time::Duration::from_millis(175));
                    if window
                        .emit(
                            "select-amount",
                            Payload {
                                message: None,
                                pos: Some(pos),
                                val: Some(val.clone()),
                            },
                        )
                        .is_ok()
                    {
                        res = Ok(());
                    }
                }
                res
            }
            None => {
                let window = WindowBuilder::new(
                    &handle,
                    "select-amount",
                    WindowUrl::App(INDEX.parse().unwrap()),
                )
                .always_on_top(true)
                .center()
                .resizable(false)
                .minimizable(false)
                .inner_size(200.0, 100.0)
                .menu(Menu::new())
                .title("Seleccione Monto")
                .build()?;
                std::thread::sleep(std::time::Duration::from_millis(400));
                let mut res = Err(AppError::IncorrectError(String::from(
                    "Error emitiendo mensaje",
                )));
                for _ in 0..8 {
                    std::thread::sleep(std::time::Duration::from_millis(175));
                    if window
                        .emit(
                            "select-amount",
                            Payload {
                                message: None,
                                pos: Some(pos),
                                val: Some(val.clone()),
                            },
                        )
                        .is_ok()
                    {
                        res = Ok(());
                    }
                }
                res
            }
        }
    }
    pub async fn open_stash_2<'a>(
        handle: tauri::AppHandle,
        sistema: State<'a, Mutex<Sistema>>,
        pos: bool,
    ) -> Res<()> {
        if sistema.lock().await.stash().len() == 0 {
            Err(AppError::IncorrectError("Stash vacío".to_string()))
        } else {
            match handle.get_window("open-stash") {
                Some(window) => {
                    window.show()?;
                    for _ in 0..7 {
                        std::thread::sleep(std::time::Duration::from_millis(250));
                        window.emit(
                            "stash",
                            Payload {
                                message: None,
                                pos: Some(pos),
                                val: None,
                            },
                        )?;
                    }
                }
                None => {
                    let win = WindowBuilder::new(
                        &handle,
                        "open-stash", /* the unique window label */
                        WindowUrl::App(INDEX.parse().unwrap()),
                    )
                    .always_on_top(true)
                    .center()
                    .resizable(false)
                    .minimizable(false)
                    .inner_size(900.0, 600.0)
                    .menu(Menu::new())
                    .title("Ventas en Stash")
                    .build()?;
                    for _ in 0..7 {
                        std::thread::sleep(std::time::Duration::from_millis(250));
                        win.emit(
                            "stash",
                            Payload {
                                message: None,
                                pos: Some(pos),
                                val: None,
                            },
                        )?;
                    }
                }
            }
            Ok(())
        }
    }
    pub fn pagar_deuda_especifica_2(
        sistema: State<Mutex<Sistema>>,
        cliente: i32,
        venta: Venta,
    ) -> Res<Venta> {
        let sis = block_on(sistema.lock());
        sis.access();
        Ok(sis.pagar_deuda_especifica(cliente, venta)?)
    }
    pub fn pagar_deuda_general_2(
        sistema: State<Mutex<Sistema>>,
        cliente: i64,
        monto: f32,
    ) -> Res<f32> {
        let sis = block_on(sistema.lock());
        sis.access();
        Ok(sis.pagar_deuda_general(cliente, monto)?)
    }
    pub fn set_cantidad_producto_venta_2(
        sistema: State<Mutex<Sistema>>,
        index: usize,
        cantidad: &str,
        pos: bool,
    ) -> Res<Venta> {
        let cantidad = cantidad.parse::<f32>()?;
        let mut sis = block_on(sistema.lock());
        sis.access();
        Ok(sis.set_cantidad_producto_venta(index, cantidad, pos)?)
    }
    pub fn set_cliente_2(sistema: State<Mutex<Sistema>>, id: i32, pos: bool) -> Res<Venta> {
        let mut sis = block_on(sistema.lock());
        sis.set_cliente(id, pos)?;
        Ok(sis.venta(pos))
    }
    pub fn set_configs_2(
        window: Window,
        sistema: State<Mutex<Sistema>>,
        configs: Config,
    ) -> Res<()> {
        let mut sis = block_on(sistema.lock());
        match sis.arc_user().rango() {
            Rango::Admin => {
                sis.set_configs(configs);
                Ok(close_window_2(window)?)
            }
            Rango::Cajero => Err(AppError::IncorrectError(DENEGADO.to_string())),
        }
    }

    pub fn stash_n_close_2(window: Window, sistema: State<Mutex<Sistema>>, pos: bool) -> Res<()> {
        let mut sis = block_on(sistema.lock());
        sis.access();
        sis.stash_sale(pos)?;
        loop {
            if window
                .emit(
                    "main",
                    Payload {
                        message: Some("dibujar venta".into()),
                        pos: None,
                        val: None,
                    },
                )
                .is_ok()
            {
                break;
            }
        }
        println!("{:#?}", sis.stash());
        Ok(close_window_2(window)?)
    }
    pub fn try_login_2(
        sistema: State<Mutex<Sistema>>,
        window: Window,
        user: User,
    ) -> Res<SistemaSH> {
        let mut sis = block_on(sistema.lock());
        // window.emit(
        //     "main",
        //     Payload {
        //         message: Some("inicio sesion".to_string()),
        //         pos: Some(false),
        //         val: None,
        //     },
        // )?;
        // println!("desde try_login {:#?}", user);
        let rango = block_on(sis.try_login(user))?;
        let menu = window
            .app_handle()
            .get_window("main")
            .unwrap()
            .menu_handle();
        match rango {
            Rango::Cajero => loop {
                if set_menus(menu.clone(), false).is_ok() {
                    break;
                }
            },
            Rango::Admin => loop {
                if set_menus(menu.clone(), true).is_ok() {
                    break;
                }
            },
        }
        // window.emit(
        //     "main",
        //     Payload {
        //         message: Some("inicio sesion".to_string()),
        //         pos: None,
        //         val: None,
        //     },
        // )?;
        // if let Some(window) = Window::get_window(&window, "main") {
        window.maximize()?;
        // }
        // close_window_2(window)?;
        let clientes = block_on(sis.get_clientes())?;
        let user = sis.arc_user().as_ref().clone();
        let conf = sis.configs().clone();
        Ok(sis.to_shared())
    }
    pub fn unstash_sale_2(
        sistema: State<Mutex<Sistema>>,
        window: Window,
        pos: bool,
        index: &str,
    ) -> Res<()> {
        let index = index.parse::<usize>()?;
        let mut sis = block_on(sistema.lock());
        sis.access();
        loop {
            if window.close().is_ok() {
                break;
            }
        }
        loop {
            if window
                .emit(
                    "main",
                    Payload {
                        message: Some(String::from("dibujar venta")),
                        pos: None,
                        val: None,
                    },
                )
                .is_ok()
            {
                break;
            }
        }
        Ok(sis.unstash_sale(pos, index)?)
    }
}
