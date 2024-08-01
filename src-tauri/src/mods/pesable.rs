use crate::mods::{
    db::{
        map::{BigIntDB, CodeDB, PesableDB},
        Mapper,
    },
    AppError, Res,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Pool, Sqlite};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pesable {
    id: i32,
    codigo: i64,
    precio_peso: f32,
    porcentaje: f32,
    costo_kilo: f32,
    descripcion: Arc<str>,
}
#[derive(Serialize, Deserialize)]
pub struct PesableSH {
    id: i32,
    codigo: [u8; 8],
    precio_peso: f32,
    descripcion: Arc<str>,
}
#[derive(Serialize, Deserialize)]
pub struct PesableSHC {
    id: i32,
    codigo: [u8; 8],
    precio_peso: f32,
    porcentaje: f32,
    costo_kilo: f32,
    descripcion: Arc<str>,
}
impl Pesable {
    pub fn build(
        id: i32,
        codigo: i64,
        precio_peso: f32,
        porcentaje: f32,
        costo_kilo: f32,
        descripcion: &str,
    ) -> Pesable {
        Pesable {
            id,
            codigo,
            precio_peso,
            porcentaje,
            costo_kilo,
            descripcion: Arc::from(descripcion),
        }
    }
    pub async fn new_to_db(&mut self, db: &Pool<Sqlite>) -> Res<String> {
        let qres:Option<BigIntDB>=sqlx::query_as!(BigIntDB,"select pesables.id as int from codigos inner join pesables on codigos.pesable = pesables.id where codigo = ?",self.codigo
        ).fetch_optional(db).await?;
        match qres {
            Some(_) => {
                return Err(AppError::ExistingError {
                    objeto: "Pesable".to_string(),
                    instancia: self.codigo.to_string(),
                })
            }
            None => {
                let qres = sqlx::query("insert into pesables values (?, ?, ?, ?, ?, ?)")
                    .bind(self.codigo)
                    .bind(self.precio_peso)
                    .bind(self.porcentaje)
                    .bind(self.costo_kilo)
                    .bind(self.descripcion.as_ref())
                    .bind(Utc::now().naive_local())
                    .execute(db)
                    .await?;
                self.id = qres.last_insert_rowid() as i32;
                Ok(String::from("Pesable agregado correctamente"))
            }
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn codigo(&self) -> &i64 {
        &self.codigo
    }
    pub fn precio_peso(&self) -> &f32 {
        &self.precio_peso
    }
    pub fn porcentaje(&self) -> &f32 {
        &self.porcentaje
    }
    pub fn costo_kilo(&self) -> &f32 {
        &self.costo_kilo
    }
    pub fn descripcion(&self) -> Arc<str> {
        Arc::clone(&self.descripcion)
    }
    pub async fn fetch_code(code: CodeDB, db: &Pool<Sqlite>) -> Res<Pesable> {
        let pes = code.pesable.unwrap();
        let qres: PesableDB = query_as!(
            PesableDB,
            r#"select id as "id:_", precio_peso as "precio_peso:_", porcentaje as "porcentaje:_",costo_kilo as "costo_kilo:_", descripcion, updated_at from pesables where id = ?"#,
            pes
        )
        .fetch_one(db)
        .await?;
        Ok(Mapper::pesable(qres, code.codigo))
    }
    pub async fn eliminar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from pesables where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => {
                sqlx::query("delete from pesables where id = ?")
                    .bind(self.id)
                    .execute(db)
                    .await?;
            }
            None => {
                return Err(AppError::NotFound {
                    objeto: String::from("Pesable"),
                    instancia: self.id.to_string(),
                })
            }
        }
        Ok(())
    }
    #[cfg(test)]
    pub fn desc(&self) -> String {
        self.descripcion.to_string()
    }
    pub async fn editar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from pesables where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => {
                if self.precio_peso == self.costo_kilo * (1.0 + self.porcentaje / 100.0) {
                    sqlx::query(
                        "update pesables set precio_peso = ?, costo_kilo = ?,
                    descripcion =?,
                    porcentaje=?,
                    updated_at=? where id = ?",
                    )
                    .bind(self.precio_peso)
                    .bind(self.costo_kilo)
                    .bind(self.descripcion.as_ref())
                    .bind(self.porcentaje)
                    .bind(Utc::now().naive_local())
                    .execute(db)
                    .await?;
                    Ok(())
                } else {
                    Err(AppError::IncorrectError(String::from(
                        "Cálculo de precio incorrecto",
                    )))
                }
            }
            None => {
                return Err(AppError::NotFound {
                    objeto: String::from("Pesable"),
                    instancia: self.id.to_string(),
                })
            }
        }
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
