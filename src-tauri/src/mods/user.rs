use crate::mods::{db::map::BigIntDB, AppError, Res};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: Arc<str>,
    nombre: Arc<str>,
    pass: i64,
    rango: Rango,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserSHC {
    pub id: Arc<str>,
    pub nombre: Arc<str>,
    pub pass: [u8; 8],
    pub rango: Rango,
}
#[derive(Serialize, Deserialize)]
pub struct UserSH {
    pub id: Arc<str>,
    pub nombre: Arc<str>,
    pub rango: Rango,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Rango {
    Admin,
    Cajero,
}
impl User {
    pub async fn new_to_db(&self, db: &Pool<Sqlite>) -> Res<()> {
        let id_2 = self.id.as_ref();
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from users where user_id = ?",
            id_2
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => Err(AppError::IncorrectError(String::from("Usuario existente"))),
            None => {
                sqlx::query("insert into users values (?, ?, ?, ?)")
                    .bind(self.id.as_ref())
                    .bind(self.nombre.as_ref())
                    .bind(self.pass)
                    .bind(self.rango.to_string())
                    .execute(db)
                    .await?;
                Ok(())
            }
        }
    }
    pub fn build(id: Arc<str>, nombre: Arc<str>, pass: i64, rango: Rango) -> User {
        User {
            id,
            pass,
            rango,
            nombre,
        }
    }
    pub fn rango(&self) -> &Rango {
        &self.rango
    }
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
    pub fn pass(&self) -> &i64 {
        &self.pass
    }
    pub fn nombre(&self) -> Arc<str> {
        Arc::clone(&self.nombre)
    }
    pub fn to_shared(&self) -> UserSH {
        UserSH {
            id: self.id.clone(),
            nombre: self.nombre.clone(),
            rango: self.rango.clone(),
        }
    }
    pub fn to_shared_complete(&self) -> UserSHC {
        UserSHC {
            id: self.id.clone(),
            nombre: self.nombre.clone(),
            pass: self.pass.to_be_bytes(),
            rango: self.rango.clone(),
        }
    }

    pub fn from_shared_complete(user: UserSHC) -> Self {
        User {
            id: Arc::from(user.id),
            nombre: Arc::from(user.nombre),
            pass: i64::from_be_bytes(user.pass),
            rango: user.rango,
        }
    }
}
impl Display for Rango {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Rango::Admin => String::from("Admin"),
            Rango::Cajero => String::from("Cajero"),
        };
        write!(f, "{}", str)
    }
}
