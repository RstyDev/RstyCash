use serde::{Deserialize, Serialize};

use super::{
    pesable::Pesable, producto::Producto, rubro::Rubro, Formato, PesableSH, PesableSHC, ProductoSH,
    ProductoSHC, RubroSHC,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Valuable {
    Prod((u8, Producto)),
    Pes((f32, Pesable)),
    Rub((u8, Rubro)),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ValuableSH {
    Prod((u8, ProductoSH)),
    Pes((f32, PesableSH)),
    Rub((u8, RubroSHC)),
}

#[derive(Serialize, Deserialize)]
pub enum ValuableSHC {
    Prod((u8, ProductoSHC)),
    Pes((f32, PesableSHC)),
    Rub((u8, RubroSHC)),
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

impl PartialEq for ValuableSH {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Prod(l0), Self::Prod(r0)) => l0.1.id == r0.1.id,
            (Self::Pes(l0), Self::Pes(r0)) => l0.1.id == r0.1.id,
            (Self::Rub(l0), Self::Rub(r0)) => l0.1.id == r0.1.id,
            _ => false,
        }
    }
}
