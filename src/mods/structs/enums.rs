use sycamore::prelude::RcSignal;

use super::{Cliente, Config, Venta};
#[derive(Debug, PartialEq, Clone)]
pub enum Pos {
    A {
        venta: RcSignal<Venta>,
        config: RcSignal<Config>,
        clientes: RcSignal<Vec<Cliente>>,
    },
    B {
        venta: RcSignal<Venta>,
        config: RcSignal<Config>,
        clientes: RcSignal<Vec<Cliente>>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Buscando {
    False {
        pos: RcSignal<Pos>,
        focus: RcSignal<bool>,
    },
    True {
        nav: RcSignal<Nav>,
        search: RcSignal<String>,
        pos: RcSignal<Pos>,
        aux: RcSignal<bool>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Nav {
    Up,
    Down,
    Enter,
    Esc,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Restante {
    Pagado(f32),
    NoPagado(RcSignal<f32>),
}

impl ToString for Restante {
    fn to_string(&self) -> String {
        match self {
            Restante::Pagado(monto) => monto.to_string(),
            Restante::NoPagado(rc_signal) => rc_signal.get().to_string(),
        }
    }
}
