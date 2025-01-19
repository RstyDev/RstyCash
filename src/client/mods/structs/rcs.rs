use super::{Caja, Cliente, Config, Proveedor, User, Venta};
use sycamore::prelude::Signal;
#[derive(Clone)]
pub struct Rcs {
    pub user: Signal<User>,
    pub caja: Signal<Caja>,
    pub config: Signal<Config>,
    pub venta_a: Signal<Venta>,
    pub venta_b: Signal<Venta>,
    pub proveedores: Signal<Vec<Proveedor>>,
    pub clientes: Signal<Vec<Cliente>>,
    pub logged: Signal<bool>,
}
