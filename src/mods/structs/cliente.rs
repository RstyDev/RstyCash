use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug, Deserialize, Default)]
pub enum Cliente {
    #[default]
    Final,
    Regular(Cli),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cli {
    pub id: i32,
    pub nombre: String,
    pub dni: i32,
    pub activo: bool,
    pub created: NaiveDateTime,
    pub limite: Cuenta,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Cuenta {
    Auth(f32),
    Unauth,
}
impl Cli {
    pub fn new(
        id: i32,
        nombre: String,
        dni: i32,
        activo: bool,
        created: NaiveDateTime,
        limite: Cuenta,
    ) -> Cli {
        Cli {
            id,
            nombre,
            dni,
            activo,
            created,
            limite,
        }
    }
}
impl PartialEq for Cuenta {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Auth(l0), Self::Auth(r0)) => l0 == r0,
            (Self::Unauth, Self::Unauth) => true,
            _ => false,
        }
    }
}
impl PartialEq for Cliente {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Regular(l0), Self::Regular(r0)) => l0.id == r0.id,

            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
