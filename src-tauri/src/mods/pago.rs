use rand::random;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedioPago {
    medio: Arc<str>,
    id: i32,
}

impl Default for MedioPago {
    fn default() -> Self {
        Self {
            medio: Arc::from("Efectivo"),
            id: 1,
        }
    }
}

impl MedioPago {
    pub fn build(medio: &str, id: i32) -> MedioPago {
        MedioPago {
            medio: Arc::from(medio),
            id,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn desc(&self) -> Arc<str> {
        Arc::clone(&self.medio)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pago {
    int_id: i32,
    medio_pago: MedioPago,
    monto: f32,
    pagado: f32,
}

impl PartialEq for Pago {
    fn eq(&self, other: &Self) -> bool {
        self.int_id == other.int_id
    }
}

impl Default for Pago {
    fn default() -> Self {
        Self {
            int_id: random(),
            medio_pago: Default::default(),
            monto: 0.0,
            pagado: 0.0,
        }
    }
}

impl Pago {
    pub fn new(medio_pago: MedioPago, monto: f32, pagado: Option<f32>) -> Pago {
        let int_id = random();
        Pago {
            medio_pago,
            monto,
            int_id,
            pagado: pagado.unwrap_or(monto),
        }
    }
    pub fn build(int_id: i32, medio_pago: MedioPago, monto: f32, pagado: f32) -> Pago {
        Pago {
            int_id,
            medio_pago,
            monto,
            pagado,
        }
    }
    pub fn medio_pago(&self) -> &MedioPago {
        &self.medio_pago
    }
    pub fn medio(&self) -> Arc<str> {
        Arc::clone(&self.medio_pago.medio)
    }
    pub fn monto(&self) -> f32 {
        self.monto
    }
    pub fn id(&self) -> i32 {
        self.int_id
    }
    pub fn pagado(&self) -> &f32 {
        &self.pagado
    }
    pub fn set_pagado(&mut self, pagado: f32) {
        self.pagado = pagado;
    }

    pub fn to_shared_complete(&self) -> Self {
        self.clone()
    }
}
