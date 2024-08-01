use serde::{Deserialize, Serialize};

use super::{pesable::Pesable, producto::Producto, rubro::Rubro, Formato};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Valuable {
    Prod((u8, Producto)),
    Pes((f32, Pesable)),
    Rub((u8, Rubro)),
}

impl Valuable {
    pub fn get_desc(&self, formato: Formato) -> String {
        match self {
            Valuable::Prod((cant, prod)) => format!("{} {}", prod.get_desc(formato), cant),
            Valuable::Pes((cant, pes)) => format!("{} {}", pes.get_desc(), cant),
            Valuable::Rub((cant, rub)) => format!("{} {}", rub.get_desc(), cant),
        }
    }
    pub fn id(&self) -> i64 {
        match self {
            Valuable::Prod((_, prod)) => prod.codigos_de_barras[0],
            Valuable::Pes((_, pes)) => pes.codigo,
            Valuable::Rub((_, rub)) => rub.codigo,
        }
    }
}
