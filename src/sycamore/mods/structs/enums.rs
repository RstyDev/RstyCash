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

impl Pos{
    pub fn is_a(&self)->bool{
        match self{
            Pos::A { .. } => true,
            Pos::B { .. } => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Buscando {
    False {
        pos: RcSignal<Pos>,
        focus: RcSignal<bool>,
        other_sale: RcSignal<Venta>,
    },
    True {
        nav: RcSignal<Nav>,
        search: RcSignal<String>,
        pos: RcSignal<Pos>,
        other_sale: RcSignal<Venta>,
        aux: RcSignal<bool>,
        focus: RcSignal<bool>,
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

impl Restante {
    pub fn pagado(&self) -> bool {
        match self {
            Restante::Pagado(_) => true,
            Restante::NoPagado(_) => false,
        }
    }
    pub fn monto(&self)->f32{
        match self{
            Restante::Pagado(monto) => *monto,
            Restante::NoPagado(rc_signal) => *rc_signal.get(),
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
