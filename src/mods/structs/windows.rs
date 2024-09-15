use sycamore::prelude::RcSignal;

use crate::mods::main_window::main_page::StateProps;

use super::User;

#[derive(Clone, Debug, PartialEq)]
pub enum Windows {
    Main(StateProps),
    Login(RcSignal<User>),
}
