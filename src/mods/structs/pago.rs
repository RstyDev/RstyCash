use serde::{Deserialize, Serialize};

use super::medio_pago::MedioPago;

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Pago {
    pub int_id: i32,
    pub medio_pago: MedioPago,
    pub monto: f32,
    pub pagado: f32,
}
impl Pago {
    pub fn new(int_id: i32, medio_pago: MedioPago, monto: f32, pagado: f32) -> Pago {
        Pago {
            int_id,
            medio_pago,
            monto,
            pagado,
        }
    }
}
impl PartialEq for Pago {
    fn eq(&self, other: &Self) -> bool {
        self.int_id == other.int_id
    }
}

impl Eq for Pago {}
