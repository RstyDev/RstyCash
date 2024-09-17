use serde::{Deserialize, Serialize};

use super::{Pago, ValuableSH};
#[derive(Serialize, Deserialize)]
pub struct AgregarPago {
    pub pago: Pago,
    pub pos: bool,
}
#[derive(Serialize, Deserialize)]
pub struct AgregarProductoAVenta {
    pub prod: ValuableSH,
    pub pos: bool,
}
#[derive(Serialize, Deserialize)]
pub struct DecrementarProductoDeVenta {
    pub index: usize,
    pub pos: bool,
}
#[derive(Serialize, Deserialize)]
pub struct EliminarProductoDeVenta {
    pub index: usize,
    pub pos: bool,
}
#[derive(Serialize, Deserialize)]
pub struct GetProductosFiltrado<'a> {
    pub filtro: &'a str,
}
#[derive(Serialize, Deserialize)]
pub struct IncrementarProductoAVenta {
    pub index: usize,
    pub pos: bool,
}
#[derive(Serialize, Deserialize)]
pub struct SetCantidadProductoVenta {
    pub index: usize,
    pub cantidad: f32,
    pub pos: bool,
}
#[derive(Serialize, Deserialize)]
pub struct SetCliente {
    pub id: i32,
    pub pos: bool,
}
