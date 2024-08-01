use super::{Caja, Cli, Config, ProveedorSH, UserSH, VentaSHC};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SistemaSH {
    pub user: UserSH,
    pub caja: Caja,
    pub clientes: Vec<Cli>,
    pub configs: Config,
    pub ventas: [VentaSHC; 2],
    pub proveedores: Vec<ProveedorSH>,
}
