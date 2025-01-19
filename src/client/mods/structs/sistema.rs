use super::{Caja, Cliente, Config, ProveedorSH, UserSH, VentaSH};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SistemaSH {
    pub user: UserSH,
    pub caja: Caja,
    pub clientes: Vec<Cliente>,
    pub configs: Config,
    pub ventas: [VentaSH; 2],
    pub proveedores: Vec<ProveedorSH>,
}

impl SistemaSH {
    pub fn new(
        user: UserSH,
        caja: Caja,
        clientes: Vec<Cliente>,
        configs: Config,
        ventas: [VentaSH; 2],
        proveedores: Vec<ProveedorSH>,
    ) -> SistemaSH {
        SistemaSH {
            user,
            caja,
            clientes,
            configs,
            ventas,
            proveedores,
        }
    }
}
