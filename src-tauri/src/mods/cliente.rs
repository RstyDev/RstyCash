use chrono::{NaiveDateTime, Utc};

use crate::mods::db::{
    map::{ClienteDB, FloatDB, IntDB, PagoDB, VentaDB},
    Mapper,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

use super::{AppError, Res, User, Venta};

#[derive(Serialize, Clone, Debug, Deserialize)]
pub enum Cliente {
    Final,
    Regular(Cli),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cli {
    dni: i32,
    nombre: Arc<str>,
    activo: bool,
    created: NaiveDateTime,
    limite: Cuenta,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Cuenta {
    Auth(f32),
    Unauth,
}
impl Cli {
    pub async fn new_to_db(db: &Pool<Sqlite>, cliente: Cli) -> Res<Cli> {
        let model: Option<ClienteDB> = sqlx::query_as!(
            ClienteDB,
            r#"select dni as "dni: _", nombre, limite as "limite: _", activo, time from clientes where dni = ? limit 1"#,
            cliente.dni
        )
        .fetch_optional(db)
        .await?;
        match model {
            Some(_) => {
                return Err(AppError::ExistingError {
                    objeto: "Cliente".to_string(),
                    instancia: cliente.dni.to_string(),
                })
            }
            None => {
                sqlx::query("insert into clientes values (?, ?, ?, ?, ?)")
                    .bind(cliente.dni)
                    .bind(cliente.nombre.as_ref())
                    .bind(match cliente.limite {
                        Cuenta::Unauth => None,
                        Cuenta::Auth(a) => Some(a),
                    })
                    .bind(cliente.activo)
                    .bind(Utc::now().naive_local())
                    .execute(db)
                    .await?;
                Ok(Cli {
                    dni: cliente.dni,
                    nombre: Arc::from(cliente.nombre.as_ref()),
                    limite: cliente.limite,
                    activo: cliente.activo,
                    created: cliente.created,
                })
            }
        }
    }
    pub fn build(
        dni: i32,
        nombre: Arc<str>,
        activo: bool,
        created: NaiveDateTime,
        limite: Option<f32>,
    ) -> Cli {
        Cli {
            dni,
            nombre,
            limite: match limite {
                Some(limit) => Cuenta::Auth(limit),
                None => Cuenta::Unauth,
            },
            activo,
            created,
        }
    }
    pub async fn existe(dni: i32, db: &Pool<Sqlite>) -> Res<bool> {
        Ok(sqlx::query_as!(
            IntDB,
            r#"select dni as "int:_" from clientes where dni = ?"#,
            dni
        )
        .fetch_optional(db)
        .await?
        .is_none())
    }
    pub fn dni(&self) -> &i32 {
        &self.dni
    }
    pub fn limite(&self) -> &Cuenta {
        &self.limite
    }
    pub fn nombre(&self) -> &str {
        self.nombre.as_ref()
    }
    pub async fn get_deuda(&self, db: &Pool<Sqlite>) -> Res<f32> {
        let model: sqlx::Result<Vec<FloatDB>> = sqlx::query_as!(
            FloatDB,
            r#"select monto as "float:_" from deudas where cliente = ? "#,
            self.dni
        )
        .fetch_all(db)
        .await;
        Ok(model?.iter().map(|e| e.float).sum::<f32>())
    }
    pub async fn get_deuda_detalle(
        &self,
        db: &Pool<Sqlite>,
        user: Option<Arc<User>>,
    ) -> Res<Vec<Venta>> {
        let mut ventas = Vec::new();
        let qres: Vec<VentaDB> = sqlx::query_as!(
            VentaDB,
            r#"select id as "id:_", time, monto_total as "monto_total:_", monto_pagado as "monto_pagado:_", cliente as "cliente:_", cerrada, paga, pos from ventas where cliente = ? and paga = ? "#,
            self.dni,
            false
        )
        .fetch_all(db)
        .await?;
        for model in qres {
            ventas.push(Mapper::venta(db, model, &user).await?)
        }
        Ok(ventas)
    }

    pub async fn pagar_deuda_especifica(
        id_cliente: i32,
        db: &Pool<Sqlite>,
        venta: Venta,
        user: &Option<Arc<User>>,
    ) -> Res<Venta> {
        let qres: Option<VentaDB> = sqlx::query_as!(
            VentaDB,
            r#"select id as "id:_", time, monto_total as "monto_total:_", monto_pagado as "monto_pagado:_", cliente as "cliente:_", cerrada, paga, pos from ventas where id = ? and cliente = ? and paga = ? "#,
            *venta.id(),
            id_cliente,
            false
        )
        .fetch_optional(db)
        .await?;
        let venta = match qres {
            Some(model) => model,
            None => return Err(AppError::IncorrectError(String::from("Id inexistente"))),
        };

        if venta.cliente == id_cliente {
            let venta = Mapper::venta(db, venta, user).await?;
            sqlx::query!(
                "update ventas set paga = ? where id = ? ",
                *venta.id(),
                true
            )
            .execute(db)
            .await?;
            Ok(venta)
        } else {
            Err(AppError::IncorrectError(String::from("Cliente Incorrecto")))
        }
    }
    pub async fn pagar_deuda_general(
        id: i64,
        db: &Pool<Sqlite>,
        mut monto_a_pagar: f32,
    ) -> Res<f32> {
        let qres: Vec<VentaDB> = sqlx::query_as!(
            VentaDB,
            r#"select id as "id:_", time, monto_total as "monto_total:_", monto_pagado as "monto_pagado:_", cliente as "cliente:_", cerrada, paga, pos from ventas where cliente = ? and paga = ? "#,
            id,
            false
        )
        .fetch_all(db)
        .await?;
        let resto = monto_a_pagar
            - qres
                .iter()
                .map(|model| model.monto_total - model.monto_pagado)
                .sum::<f32>();
        for venta in qres {
            if monto_a_pagar <= 0.0 {
                break;
            }

            let models: Vec<PagoDB> = sqlx::query_as!(
                PagoDB,
                r#"select id as "id:_", medio_pago as "medio_pago:_", monto as "monto:_", pagado as "pagado:_", venta as "venta:_" from pagos where venta = ? and medio_pago = ? "#,
                venta.id,
                0
            )
            .fetch_all(db)
            .await?;
            let mut completados: u8 = 0;
            for i in 0..models.len() {
                if monto_a_pagar <= 0.0 {
                    break;
                }
                if models[i].pagado < models[i].monto {
                    if monto_a_pagar >= (models[i].monto - models[i].pagado) {
                        monto_a_pagar -= models[i].monto - models[i].pagado;
                        completados += 1;
                        sqlx::query("update pagos set pagado = ? where id =?")
                            .bind(models[i].monto)
                            .bind(id)
                            .execute(db)
                            .await?;
                    } else {
                        sqlx::query("update pagos set pagado = ? where id = ?")
                            .bind(models[i].pagado + monto_a_pagar)
                            .bind(id)
                            .execute(db)
                            .await?;
                        monto_a_pagar = 0.0;
                    }
                }
            }
            if completados == models.len() as u8 {
                sqlx::query("update ventas set paga = ? where id = ?")
                    .bind(true)
                    .bind(venta.id)
                    .execute(db)
                    .await?;
            }
        }
        Ok(resto)
    }
}

impl Cliente {
    pub fn new(cli: Option<Cli>) -> Cliente {
        match cli {
            Some(a) => Cliente::Regular(a),
            None => Cliente::Final,
        }
    }
    pub fn to_shared_complete(&self) -> Self {
        self.clone()
    }
    pub async fn insert_final(db: &Pool<Sqlite>) -> Res<()> {
        if sqlx::query_as!(
            IntDB,
            r#"select dni as "int:_" from clientes where nombre = ?"#,
            "Final"
        )
        .fetch_optional(db)
        .await?
        .is_none()
        {
            sqlx::query("insert into clientes values (?, ?, ?, ?, ?)")
                .bind(1)
                .bind("Final")
                .bind(None::<f32>)
                .bind(true)
                .bind(NaiveDateTime::MIN)
                .execute(db)
                .await?;
        }
        Ok(())
    }
}

impl Default for Cliente {
    fn default() -> Self {
        Cliente::Final
    }
}
