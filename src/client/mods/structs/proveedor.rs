use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    pub id: i32,
    pub nombre: Arc<str>,
    pub contacto: Option<i64>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProveedorSH {
    pub id: i32,
    pub nombre: Arc<str>,
    contacto: Option<[u8; 8]>,
}
impl Proveedor {
    pub fn new(id: i32, nombre: Arc<str>, contacto: Option<i64>) -> Proveedor {
        Proveedor {
            id,
            nombre,
            contacto,
        }
    }
    pub fn to_shared_complete(&self) -> ProveedorSH {
        ProveedorSH {
            id: self.id,
            nombre: self.nombre.clone(),
            contacto: self.contacto.map(|c| c.to_be_bytes()),
        }
    }
    pub fn from_shared_complete(proveedor: ProveedorSH) -> Self {
        Proveedor {
            id: proveedor.id,
            nombre: proveedor.nombre,
            contacto: proveedor.contacto.map(|c| i64::from_be_bytes(c)),
        }
    }
}
