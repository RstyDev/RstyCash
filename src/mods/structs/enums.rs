use std::rc::Rc;

use sycamore::prelude::{RcSignal, ReadSignal, Signal};

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
pub enum Buscando{
    False {
        venta: RcSignal<Venta>,
        config: RcSignal<Config>,
        clientes: RcSignal<Vec<Cliente>>,
        pos: RcSignal<Pos>,
    },
    True {
        search: RcSignal<String>,
        venta: RcSignal<Venta>,
    },
}
