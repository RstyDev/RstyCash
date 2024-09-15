use serde::{Deserialize, Serialize};

use super::{
    pesable::Pesable, producto::Producto, rubro::Rubro, Formato, PesableSH, PesableSHC, ProductoSH,
    ProductoSHC, RubroSHC,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            Valuable::Prod((_, prod)) => format!("{}", prod.get_desc(formato)),
            Valuable::Pes((_, pes)) => format!("{}", pes.get_desc()),
            Valuable::Rub((_, rub)) => format!("{}", rub.get_desc()),
        }
    }
    pub fn id(&self) -> i64 {
        match self {
            Valuable::Prod((_, prod)) => prod.codigos_de_barras[0],
            Valuable::Pes((_, pes)) => pes.codigo,
            Valuable::Rub((_, rub)) => rub.codigo,
        }
    }
    pub fn get_f_cant(&self) -> f32 {
        match self {
            Valuable::Prod((c, _)) => *c as f32,
            Valuable::Pes((c, _)) => *c,
            Valuable::Rub((c, _)) => *c as f32,
        }
    }
    pub fn get_shared_code(&self) -> [u8; 8] {
        match self {
            Valuable::Prod((_, p)) => p.codigos_de_barras[0].to_be_bytes(),
            Valuable::Pes((_, p)) => p.codigo.to_be_bytes(),
            Valuable::Rub((_, r)) => r.codigo.to_be_bytes(),
        }
    }
    pub fn get_unit_price(&self) -> f32 {
        match self {
            Valuable::Prod((_, prod)) => prod.precio_venta,
            Valuable::Pes((_, pes)) => pes.precio_peso,
            Valuable::Rub((_, rub)) => rub.monto.unwrap_or_default(),
        }
    }
    pub fn get_total_price(&self) -> f32 {
        match self {
            Valuable::Prod((c, p)) => *c as f32 * p.precio_venta,
            Valuable::Pes((c, p)) => *c * p.precio_peso,
            Valuable::Rub((c, r)) => *c as f32 * r.monto.unwrap_or_default(),
        }
    }
}

impl PartialEq for Valuable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Prod(l0), Self::Prod(r0)) => l0.0 == r0.0 && l0.1.id == r0.1.id,
            (Self::Pes(l0), Self::Pes(r0)) => l0.0 == r0.0 && l0.1.id == r0.1.id,
            (Self::Rub(l0), Self::Rub(r0)) => l0.0 == r0.0 && l0.1.id == r0.1.id,
            _ => false,
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
