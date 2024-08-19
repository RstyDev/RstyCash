use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum Formato {
    #[default]
    Tmv,
    Mtv,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum Mayusculas {
    #[default]
    Upper,
    Lower,
    Camel,
}
impl Display for Mayusculas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mayusculas::Upper => "Upper",
                Mayusculas::Lower => "Lower",
                Mayusculas::Camel => "Camel",
            }
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Config {
    pub politica_redondeo: f32,
    pub formato_producto: Formato,
    pub modo_mayus: Mayusculas,
    pub cantidad_productos: u8,
    pub medios_pago: Vec<Arc<str>>,
}

impl Config {
    pub fn new(
        politica_redondeo: f32,
        formato_producto: Formato,
        modo_mayus: Mayusculas,
        cantidad_productos: u8,
        medios_pago: Vec<Arc<str>>,
    ) -> Config {
        Config {
            politica_redondeo,
            formato_producto,
            modo_mayus,
            cantidad_productos,
            medios_pago,
        }
    }
}
