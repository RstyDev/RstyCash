use super::{MedioPago, Res};
use crate::mods::db::map::{ConfigDB, MedioPagoDB};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    politica_redondeo: f32,
    formato_producto: Formato,
    modo_mayus: Mayusculas,
    cantidad_productos: u8,
    medios_pago: Vec<MedioPago>,
}

impl Config {
    pub async fn get_or_def(db: &Pool<Sqlite>) -> Res<Config> {
        let res: Option<ConfigDB> = sqlx::query_as!(ConfigDB, r#"select id as "id:_", politica as "politica:_", formato, mayus, cantidad as "cantidad:_" from config limit 1"#)
            .fetch_optional(db)
            .await?;
        match res {
            Some(conf) => {
                let medios: sqlx::Result<Vec<MedioPagoDB>> = sqlx::query_as!(
                    MedioPagoDB,
                    r#"select id as "id:_", medio from medios_pago "#
                )
                .fetch_all(db)
                .await;
                let medios = medios?
                    .iter()
                    .map(|med| MedioPago::build(&med.medio, med.id))
                    .collect::<Vec<MedioPago>>();
                Ok(Config::build(
                    conf.politica,
                    conf.formato.as_str(),
                    conf.mayus.as_str(),
                    conf.cantidad,
                    medios,
                ))
            }
            None => {
                let conf = Config::default();
                sqlx::query("insert into config values (?, ?, ?, ?, ?)")
                    .bind(1)
                    .bind(conf.politica())
                    .bind(conf.formato().to_string())
                    .bind(conf.modo_mayus().to_string())
                    .bind(conf.cantidad_productos())
                    .execute(db)
                    .await?;
                Ok(conf)
            }
        }
    }
    pub fn build(
        politica_redondeo: f32,
        formato_producto: &str,
        modo_mayus: &str,
        cantidad_productos: u8,
        medios_pago: Vec<MedioPago>,
    ) -> Config {
        let formato_producto = match formato_producto {
            "Tmv" => Formato::Tmv,
            "Mtv" => Formato::Mtv,
            _ => panic!("solo hay Tmv y Mtv"),
        };
        let modo_mayus = match modo_mayus {
            "Upper" => Mayusculas::Upper,
            "Lower" => Mayusculas::Lower,
            "Camel" => Mayusculas::Camel,
            _ => panic!("solo hay Upper, Lower y Camel"),
        };
        Config {
            politica_redondeo,
            formato_producto,
            modo_mayus,
            cantidad_productos,
            medios_pago,
        }
    }
    pub fn cantidad_productos(&self) -> &u8 {
        &self.cantidad_productos
    }
    pub fn medios_pago(&self) -> &Vec<MedioPago> {
        &self.medios_pago
    }
    pub fn politica(&self) -> f32 {
        self.politica_redondeo
    }
    pub fn formato(&self) -> &Formato {
        &self.formato_producto
    }
    pub fn modo_mayus(&self) -> Mayusculas {
        self.modo_mayus.clone()
    }
    pub fn to_shared_complete(&self) -> Self {
        self.clone()
    }
}
impl Default for Config {
    fn default() -> Self {
        Config {
            politica_redondeo: 10.0,
            formato_producto: Formato::default(),
            modo_mayus: Mayusculas::default(),
            cantidad_productos: 20,
            medios_pago: vec![
                MedioPago::build("Efectivo", 1),
                MedioPago::build("Crédito", 2),
                MedioPago::build("Débito", 3),
            ],
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Formato {
    #[default]
    Tmv,
    Mtv,
}
impl Display for Formato {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Formato::Tmv => String::from("Tmv"),
            Formato::Mtv => String::from("Mtv"),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Mayusculas {
    #[default]
    Upper,
    Lower,
    Camel,
}
impl Display for Mayusculas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Mayusculas::Upper => String::from("Upper"),
            Mayusculas::Lower => String::from("Lower"),
            Mayusculas::Camel => String::from("Camel"),
        };
        write!(f, "{}", str)
    }
}
