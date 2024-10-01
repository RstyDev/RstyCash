use crate::mods::structs::Formato;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Producto {
    pub id: i32,
    pub codigos_de_barras: [i64; 3],
    pub precio_venta: f32,
    pub porcentaje: f32,
    pub precio_costo: f32,
    pub tipo_producto: Arc<str>,
    pub marca: Arc<str>,
    pub variedad: Arc<str>,
    pub presentacion: Presentacion,
    pub proveedores: Vec<RelacionProdProv>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProductoSH {
    pub id: i32,
    pub codigo_de_barras: [u8; 8],
    pub precio_venta: f32,
    pub tipo_producto: Arc<str>,
    pub marca: Arc<str>,
    pub variedad: Arc<str>,
    pub presentacion: Presentacion,
}
#[derive(Serialize, Deserialize)]
pub struct ProductoSHC {
    pub id: i32,
    pub codigos_de_barras: [[u8; 8]; 3],
    pub precio_venta: f32,
    pub porcentaje: f32,
    pub precio_costo: f32,
    pub tipo_producto: Arc<str>,
    pub marca: Arc<str>,
    pub variedad: Arc<str>,
    pub presentacion: Presentacion,
    pub proveedores: Vec<RelacionProdProv>,
}
impl Producto {
    pub fn new(
        id: i32,
        codigos_de_barras: [i64; 3],
        precio_venta: f32,
        porcentaje: f32,
        precio_costo: f32,
        tipo_producto: Arc<str>,
        marca: Arc<str>,
        variedad: Arc<str>,
        presentacion: Presentacion,
        proveedores: Vec<RelacionProdProv>,
    ) -> Producto {
        Producto {
            id,
            codigos_de_barras,
            precio_venta,
            porcentaje,
            precio_costo,
            tipo_producto,
            marca,
            variedad,
            presentacion,
            proveedores,
        }
    }
    pub fn get_desc(&self, formato: Formato) -> String {
        match formato {
            Formato::Tmv => format!(
                "{} {} {} {} {}",
                self.tipo_producto.as_ref(),
                self.marca.as_ref(),
                self.variedad.as_ref(),
                self.presentacion.get_cantidad(),
                self.presentacion.get_string().as_str()
            ),
            Formato::Mtv => format!(
                "{} {} {} {} {}",
                self.marca.as_ref(),
                self.tipo_producto.as_ref(),
                self.variedad.as_ref(),
                self.presentacion.get_cantidad(),
                self.presentacion.get_string().as_str()
            ),
        }
    }
    pub fn to_shared(&self, codigo: i64) -> ProductoSH {
        ProductoSH {
            id: self.id,
            codigo_de_barras: match self.codigos_de_barras.iter().find(|cod| **cod == codigo) {
                Some(&a) => a.to_be_bytes(),
                None => 0i64.to_be_bytes(),
            },
            precio_venta: self.precio_venta,
            tipo_producto: self.tipo_producto.clone(),
            marca: self.marca.clone(),
            variedad: self.variedad.clone(),
            presentacion: self.presentacion.clone(),
        }
    }
    pub fn to_shared_complete(&self) -> ProductoSHC {
        ProductoSHC {
            id: self.id,
            codigos_de_barras: [
                self.codigos_de_barras[0].to_be_bytes(),
                self.codigos_de_barras[1].to_be_bytes(),
                self.codigos_de_barras[2].to_be_bytes(),
            ],
            precio_venta: self.precio_venta,
            porcentaje: self.porcentaje,
            precio_costo: self.precio_costo,
            tipo_producto: self.tipo_producto.clone(),
            marca: self.marca.clone(),
            variedad: self.variedad.clone(),
            presentacion: self.presentacion.clone(),
            proveedores: self.proveedores.clone(),
        }
    }
    pub fn from_shared_complete(producto: ProductoSHC) -> Self {
        Producto {
            id: producto.id,
            codigos_de_barras: [
                i64::from_be_bytes(producto.codigos_de_barras[0]),
                i64::from_be_bytes(producto.codigos_de_barras[1]),
                i64::from_be_bytes(producto.codigos_de_barras[2]),
            ],
            precio_venta: producto.precio_venta,
            porcentaje: producto.porcentaje,
            precio_costo: producto.precio_costo,
            tipo_producto: producto.tipo_producto,
            marca: producto.marca,
            variedad: producto.variedad,
            presentacion: producto.presentacion,
            proveedores: producto.proveedores,
        }
    }
}

impl PartialEq for Producto {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl PartialEq for ProductoSH {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RelacionProdProv {
    pub proveedor: i32,
    pub codigo_interno: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Presentacion {
    Gr(f32),
    Un(u16),
    Lt(f32),
    Ml(u16),
    CC(u16),
    Kg(f32),
}

impl RelacionProdProv {
    pub fn new(proveedor: i32, codigo_interno: Option<i32>) -> RelacionProdProv {
        RelacionProdProv {
            proveedor,
            codigo_interno,
        }
    }
}

impl Presentacion {
    pub fn new(presentacion: &str, cantidad: f32) -> Presentacion {
        match presentacion {
            "Gr" => Presentacion::Gr(cantidad),
            "Un" => Presentacion::Un(cantidad as u16),
            "Lt" => Presentacion::Lt(cantidad),
            "Ml" => Presentacion::Ml(cantidad as u16),
            "CC" => Presentacion::CC(cantidad as u16),
            "Kg" => Presentacion::Kg(cantidad),
            _ => panic!("Presentacion Inexistente"),
        }
    }
    pub fn get_cantidad(&self) -> f32 {
        match self {
            Presentacion::Gr(c) => *c,
            Presentacion::Un(c) => *c as f32,
            Presentacion::Lt(c) => *c,
            Presentacion::Ml(c) => *c as f32,
            Presentacion::CC(c) => *c as f32,
            Presentacion::Kg(c) => *c,
        }
    }
    pub fn get_string(&self) -> String {
        match self {
            Presentacion::Gr(_) => String::from("Gr"),
            Presentacion::Un(_) => String::from("Un"),
            Presentacion::Lt(_) => String::from("Lt"),
            Presentacion::Ml(_) => String::from("Ml"),
            Presentacion::CC(_) => String::from("CC"),
            Presentacion::Kg(_) => String::from("Kg"),
        }
    }
}
