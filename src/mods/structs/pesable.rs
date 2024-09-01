use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pesable {
    pub id: i32,
    pub codigo: i64,
    pub precio_peso: f32,
    pub porcentaje: f32,
    pub costo_kilo: f32,
    pub descripcion: Arc<str>,
}
#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct PesableSH {
    pub id: i32,
    pub codigo: [u8; 8],
    pub precio_peso: f32,
    pub descripcion: Arc<str>,
}
#[derive(Serialize, Deserialize)]
pub struct PesableSHC {
    pub id: i32,
    pub codigo: [u8; 8],
    pub precio_peso: f32,
    pub porcentaje: f32,
    pub costo_kilo: f32,
    pub descripcion: Arc<str>,
}
impl Pesable {
    pub fn new(
        id: i32,
        codigo: i64,
        precio_peso: f32,
        porcentaje: f32,
        costo_kilo: f32,
        descripcion: Arc<str>,
    ) -> Pesable {
        Pesable {
            id,
            codigo,
            precio_peso,
            porcentaje,
            costo_kilo,
            descripcion,
        }
    }
    pub fn get_desc(&self) -> String {
        self.descripcion.to_string()
    }
    pub fn to_shared(&self) -> PesableSH {
        PesableSH {
            id: self.id,
            codigo: self.codigo.to_be_bytes(),
            precio_peso: self.precio_peso,
            descripcion: self.descripcion.clone(),
        }
    }

    pub fn to_shared_complete(&self) -> PesableSHC {
        PesableSHC {
            id: self.id,
            codigo: self.codigo.to_be_bytes(),
            precio_peso: self.precio_peso,
            porcentaje: self.porcentaje,
            costo_kilo: self.costo_kilo,
            descripcion: self.descripcion.clone(),
        }
    }
    pub fn from_shared_complete(pesable: PesableSHC) -> Self {
        Pesable {
            id: pesable.id,
            codigo: i64::from_be_bytes(pesable.codigo),
            precio_peso: pesable.precio_peso,
            porcentaje: pesable.porcentaje,
            costo_kilo: pesable.costo_kilo,
            descripcion: pesable.descripcion,
        }
    }
}

impl PartialEq for Pesable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl PartialEq for PesableSH {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
