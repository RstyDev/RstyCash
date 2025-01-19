use std::iter::Product;
use sycamore::prelude::Signal;

use crate::client::mods::main_window::main_page::StateProps;

use super::User;

#[derive(Clone, Debug, PartialEq)]
pub enum Windows {
    Main(StateProps),
    Login(Signal<User>),
}
