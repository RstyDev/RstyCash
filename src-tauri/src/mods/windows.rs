use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Windows {
    Main,
    Login,
}
