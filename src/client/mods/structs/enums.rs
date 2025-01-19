use sycamore::prelude::Signal;

use super::{Cliente, Config, Venta};
#[derive(Debug, PartialEq, Clone)]
pub enum Pos {
    A {
        venta: Signal<Venta>,
        config: Signal<Config>,
        clientes: Signal<Vec<Cliente>>,
    },
    B {
        venta: Signal<Venta>,
        config: Signal<Config>,
        clientes: Signal<Vec<Cliente>>,
    },
}

impl Pos {
    pub fn is_a(&self) -> bool {
        match self {
            Pos::A { .. } => true,
            Pos::B { .. } => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Buscando {
    False {
        pos: Signal<Pos>,
        focus: Signal<bool>,
        other_sale: Signal<Venta>,
    },
    True {
        nav: Signal<Nav>,
        search: Signal<String>,
        pos: Signal<Pos>,
        other_sale: Signal<Venta>,
        aux: Signal<bool>,
        focus: Signal<bool>,
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
    NoPagado(Signal<f32>),
}

impl Restante {
    pub fn pagado(&self) -> bool {
        match self {
            Restante::Pagado(_) => true,
            Restante::NoPagado(_) => false,
        }
    }
    pub fn monto(&self) -> f32 {
        match self {
            Restante::Pagado(monto) => *monto,
            Restante::NoPagado(signal) => signal.get(),
        }
    }
}

impl ToString for Restante {
    fn to_string(&self) -> String {
        match self {
            Restante::Pagado(monto) => format!("{:.2}", monto),
            Restante::NoPagado(rc_signal) => format!("{:.2}", rc_signal.get()),
        }
    }
}
