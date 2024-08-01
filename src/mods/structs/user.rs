use serde::{Deserialize, Serialize};
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn get_hash(pass: &str) -> i64 {
    let mut h = DefaultHasher::new();
    pass.hash(&mut h);
    h.finish() as i64
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub nombre: String,
    pub pass: i64,
    pub rango: Rango,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSH {
    pub id: String,
    pub nombre: String,
    pub rango: Rango,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserSHC {
    pub id: String,
    pub nombre: String,
    pub pass: [u8; 8],
    pub rango: Rango,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            nombre: String::from(""),
            pass: i64::default(),
            rango: Default::default(),
        }
    }
}
impl User {
    pub fn new(id: String, pass: &str) -> User {
        User {
            id,
            nombre: String::new(),
            pass: get_hash(pass),
            rango: Rango::Cajero,
        }
    }
    pub fn set_nombre(&mut self, nombre: String) {
        self.nombre = nombre;
    }
    pub fn to_shared(self) -> UserSH {
        UserSH {
            id: self.id,
            nombre: self.nombre,
            rango: self.rango,
        }
    }
    pub fn to_shared_complete(&self) -> UserSHC {
        UserSHC {
            id: self.id.clone(),
            nombre: self.nombre.to_string(),
            pass: self.pass.to_be_bytes(),
            rango: self.rango.clone(),
        }
    }
    pub fn from_shared(user: UserSH) -> Self {
        User {
            id: user.id,
            nombre: user.nombre,
            pass: i64::default(),
            rango: user.rango,
        }
    }
    pub fn from_shared_complete(user: UserSHC) -> Self {
        User {
            id: user.id,
            nombre: user.nombre,
            pass: i64::from_be_bytes(user.pass),
            rango: user.rango,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum Rango {
    #[default]
    Admin,
    Cajero,
}
