use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rubro {
    pub id: i32,
    pub codigo: i64,
    pub monto: Option<f32>,
    pub descripcion: Arc<str>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubroSH {
    id: i32,
    codigo: [u8; 8],
    monto: Option<f32>,
    descripcion: Arc<str>,
}
impl Rubro {
    pub fn new(id: i32, codigo: i64, monto: Option<f32>, descripcion: Arc<str>) -> Rubro {
        Rubro {
            id,
            codigo,
            monto,
            descripcion,
        }
    }
    pub fn get_desc(&self) -> String {
        self.descripcion.to_string()
    }
    pub fn to_shared_complete(&self) -> RubroSH {
        RubroSH {
            id: self.id,
            codigo: self.codigo.to_be_bytes(),
            monto: self.monto,
            descripcion: self.descripcion.clone(),
        }
    }
    pub fn from_shared_complete(rubro: RubroSH) -> Self {
        Rubro {
            id: rubro.id,
            codigo: i64::from_be_bytes(rubro.codigo),
            monto: rubro.monto,
            descripcion: rubro.descripcion,
        }
    }
}

impl PartialEq for Rubro {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
