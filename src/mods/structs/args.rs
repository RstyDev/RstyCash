use serde::{Deserialize, Serialize};

use super::ValuableSH;
#[derive(Serialize, Deserialize)]
pub struct GetProductosFiltrado<'a> {
    pub filtro: &'a str,
}
#[derive(Serialize, Deserialize)]
pub struct AgregarProductoAVenta {
    pub prod: ValuableSH,
    pub pos: bool,
}
#[derive(Serialize, Deserialize)]
pub struct EliminarProductoDeVenta {
    pub code: [u8; 8],
    pub pos: bool,
}
#[derive(Serialize, Deserialize)]
pub struct IncrementarProductoAVenta{
    pub code: [u8;8],
    pub pos: bool,
}
