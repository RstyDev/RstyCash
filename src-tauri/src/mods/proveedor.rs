use crate::mods::{db::map::BigIntDB, AppError, Res};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    id: i32,
    nombre: Arc<str>,
    contacto: Option<i64>,
}
#[derive(Serialize, Deserialize)]
pub struct ProveedorSH {
    pub id: i32,
    pub nombre: Arc<str>,
    contacto: Option<[u8; 8]>,
}

impl Proveedor {
    pub async fn new_to_db(proveedor: Proveedor, db: &Pool<Sqlite>) -> Res<Proveedor> {
        let nombre = proveedor.nombre.as_ref();
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from proveedores where nombre = ?",
            nombre
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => Err(AppError::ExistingError {
                objeto: String::from("Proveedor"),
                instancia: proveedor.nombre.to_string(),
            }),
            None => {
                let qres = sqlx::query("insert into proveedores values (?, ?, ?)")
                    .bind(proveedor.nombre.as_ref())
                    .bind(proveedor.contacto)
                    .bind(Utc::now().naive_local())
                    .execute(db)
                    .await?;
                Ok(Proveedor::build(
                    qres.last_insert_rowid() as i32,
                    proveedor.nombre.as_ref(),
                    proveedor.contacto,
                ))
            }
        }
    }
    pub fn build(id: i32, nombre: &str, contacto: Option<i64>) -> Self {
        Proveedor {
            id,
            nombre: Arc::from(nombre),
            contacto,
        }
    }
    pub fn nombre(&self) -> Arc<str> {
        Arc::clone(&self.nombre)
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn contacto(&self) -> &Option<i64> {
        &self.contacto
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
impl Display for Proveedor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res;
        match self.contacto {
            Some(a) => res = format!("{} {a}", self.nombre),
            None => res = self.nombre.to_string(),
        }
        write!(f, "{}", res)
    }
}
