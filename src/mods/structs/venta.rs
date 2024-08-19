use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::{cliente::Cliente, pago::Pago, user::User, valuable::Valuable, UserSH};

#[derive(Debug, Clone, Default)]
pub struct Venta {
    pub id: i32,
    pub monto_total: f32,
    pub productos: Vec<Valuable>,
    pub pagos: Vec<Pago>,
    pub monto_pagado: f32,
    pub vendedor: Option<User>,
    pub cliente: Cliente,
    pub paga: bool,
    pub cerrada: bool,
    pub time: NaiveDateTime,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VentaSHC {
    id: i32,
    monto_total: f32,
    productos: Vec<Valuable>,
    pagos: Vec<Pago>,
    monto_pagado: f32,
    vendedor: Option<Arc<UserSH>>,
    cliente: Cliente,
    paga: bool,
    cerrada: bool,
    time: NaiveDateTime,
}
impl Venta {
    pub fn new(
        id: i32,
        monto_total: f32,
        productos: Vec<Valuable>,
        pagos: Vec<Pago>,
        monto_pagado: f32,
        vendedor: Option<User>,
        cliente: Cliente,
        paga: bool,
        cerrada: bool,
        time: NaiveDateTime,
    ) -> Venta {
        Venta {
            id,
            monto_total,
            productos,
            pagos,
            monto_pagado,
            vendedor,
            cliente,
            paga,
            cerrada,
            time,
        }
    }
    pub fn to_shared_complete(&self) -> VentaSHC {
        VentaSHC {
            id: self.id,
            monto_total: self.monto_total,
            productos: self.productos.clone(),
            pagos: self.pagos.clone(),
            monto_pagado: self.monto_pagado,
            vendedor: self
                .vendedor
                .as_ref()
                .map(|u| Arc::from(u.clone().to_shared())),
            cliente: self.cliente.clone(),
            paga: self.paga,
            cerrada: self.cerrada,
            time: self.time,
        }
    }
    pub fn from_shared_complete(venta: VentaSHC) -> Self {
        Venta {
            id: venta.id,
            monto_total: venta.monto_total,
            productos: venta.productos,
            pagos: venta.pagos,
            monto_pagado: venta.monto_pagado,
            vendedor: venta
                .vendedor
                .map(|u| User::from_shared(u.as_ref().clone())),
            cliente: venta.cliente,
            paga: venta.paga,
            cerrada: venta.cerrada,
            time: venta.time,
        }
    }
}
impl PartialEq for Venta{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}