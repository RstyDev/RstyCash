use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Caja {
    pub id: i32,
    pub inicio: NaiveDateTime,
    pub cierre: Option<NaiveDateTime>,
    pub ventas_totales: f32,
    pub monto_inicio: f32,
    pub monto_cierre: Option<f32>,
    pub cajero: Option<Arc<str>>,
    pub totales: HashMap<Arc<str>, f32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Movimiento {
    Ingreso {
        descripcion: Option<Arc<str>>,
        monto: f32,
    },
    Egreso {
        descripcion: Option<Arc<str>>,
        monto: f32,
    },
}
