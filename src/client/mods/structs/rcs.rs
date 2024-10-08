use super::{Caja, Cliente, Config, Proveedor, User, Venta};
use sycamore::prelude::RcSignal;
#[derive(Clone)]
pub struct Rcs {
    pub user: RcSignal<User>,
    pub caja: RcSignal<Caja>,
    pub config: RcSignal<Config>,
    pub venta_a: RcSignal<Venta>,
    pub venta_b: RcSignal<Venta>,
    pub proveedores: RcSignal<Vec<Proveedor>>,
    pub clientes: RcSignal<Vec<Cliente>>,
    pub logged: RcSignal<bool>,
}
