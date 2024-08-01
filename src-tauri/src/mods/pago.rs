use crate::mods::db::map::MedioPagoDB;
use rand::random;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use tauri::async_runtime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedioPago {
    medio: Arc<str>,
    id: i32,
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
    pub fn def(db: &Pool<Sqlite>) -> Self {
        let medio = async_runtime::block_on(async { medio_from_db("Efectivo", db).await });

        let medio_pago = MedioPago {
            medio: Arc::from(medio.medio),
            id: medio.id,
        };
        let int_id = random();
        Pago {
            medio_pago,
            monto: 0.0,
            int_id,
            pagado: 0.0,
        }
    }
    pub fn to_shared_complete(&self) -> Self {
        self.clone()
    }
}

pub async fn medio_from_db(medio: &str, db: &Pool<Sqlite>) -> MedioPagoDB {
    let model: sqlx::Result<Option<MedioPagoDB>> = sqlx::query_as!(
        MedioPagoDB,
        r#"select id as "id:_", medio from medios_pago where medio = ? "#,
        medio
    )
    .fetch_optional(db)
    .await;
    model.unwrap().unwrap()
}
