use crate::mods::{
    db::{
        map::{BigIntDB, CodeDB, RubroDB},
        Mapper,
    },
    redondeo,
    valuable::ValuableTrait,
    AppError, Res,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Pool, Sqlite};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rubro {
    id: i32,
    codigo: i64,
    monto: Option<f32>,
    descripcion: Arc<str>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubroSH {
    id: i32,
    codigo: [u8; 8],
    monto: Option<f32>,
    descripcion: Arc<str>,
}

impl Rubro {
    pub fn build(id: i32, codigo: i64, monto: Option<f32>, descripcion: &str) -> Rubro {
        Rubro {
            id,
            codigo,
            monto,
            descripcion: Arc::from(descripcion),
        }
    }
    pub async fn new_to_db(&self, db: &Pool<Sqlite>) -> Res<Rubro> {
        let qres = sqlx::query("insert into rubros values (?, ?)")
            .bind(self.descripcion().as_ref())
            .bind(Utc::now().naive_local())
            .execute(db)
            .await?;
        sqlx::query("insert into codigos (codigo, rubro) values (?, ?)")
            .bind(self.codigo())
            .bind(qres.last_insert_rowid())
            .execute(db)
            .await?;
        Ok(Rubro::build(
            qres.last_insert_rowid() as i32,
            *self.codigo(),
            self.monto().copied(),
            self.descripcion().as_ref(),
        ))
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn monto(&self) -> Option<&f32> {
        self.monto.as_ref()
    }
    pub fn codigo(&self) -> &i64 {
        &self.codigo
    }
    pub fn descripcion(&self) -> Arc<str> {
        Arc::clone(&self.descripcion)
    }
    pub async fn fetch_code(code: CodeDB, db: &Pool<Sqlite>) -> Res<Rubro> {
        let rub = code.rubro.unwrap();
        let qres:RubroDB=query_as!(RubroDB,r#"select id as "id:_", descripcion as "descripcion:_", updated_at as "updated_at:_" from rubros where id = ? "#,rub).fetch_one(db).await?;
        Ok(Mapper::rubro(qres, code.codigo))
    }
    #[cfg(test)]
    pub fn desc(&self) -> String {
        self.descripcion.to_string()
    }
    pub async fn eliminar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from rubros where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => {
                sqlx::query("delete from rubros where id = ?")
                    .bind(self.id)
                    .execute(db)
                    .await?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                objeto: String::from("Rubro"),
                instancia: self.id.to_string(),
            }),
        }
    }
    pub async fn editar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from rubros where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => {
                sqlx::query("update codigos set codigo = ? where rubro = ?")
                    .bind(self.codigo)
                    .bind(self.id)
                    .execute(db)
                    .await?;
                sqlx::query("update rubros set descripcion = ?, updated_at = ? where id = ?")
                    .bind(self.descripcion.as_ref())
                    .bind(Utc::now().naive_local())
                    .bind(self.id)
                    .execute(db)
                    .await?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                objeto: String::from("Rubro"),
                instancia: self.id.to_string(),
            }),
        }
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
impl ValuableTrait for Rubro {
    fn redondear(&self, politica: &f32) -> Rubro {
        match &self.monto {
            Some(a) => Rubro {
                id: self.id,
                codigo: self.codigo,
                monto: Some(redondeo(politica, *a)),
                descripcion: self.descripcion.clone(),
            },
            None => self.clone(),
        }
    }
}
