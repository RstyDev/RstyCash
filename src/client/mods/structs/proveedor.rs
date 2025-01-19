use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    pub id: i32,
    pub nombre: String,
    pub contacto: Option<i64>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProveedorSH {
    pub id: i32,
    pub nombre: String,
    contacto: Option<[u8; 8]>,
}
impl Proveedor {
    pub fn new(id: i32, nombre: String, contacto: Option<i64>) -> Proveedor {
        Proveedor {
            id,
            nombre,
            contacto,
        }
    }
    pub fn to_shared(&self) -> ProveedorSH {
        ProveedorSH {
            id: self.id,
            nombre: self.nombre.clone(),
            contacto: self.contacto.map(|c| c.to_be_bytes()),
        }
    }
    pub fn from_shared(proveedor: ProveedorSH) -> Self {
        Proveedor {
            id: proveedor.id,
            nombre: proveedor.nombre,
            contacto: proveedor.contacto.map(|c| i64::from_be_bytes(c)),
        }
    }
}
