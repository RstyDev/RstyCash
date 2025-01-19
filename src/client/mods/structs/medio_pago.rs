use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct MedioPago {
    pub medio: String,
    pub id: i32,
}

impl MedioPago {
    pub fn new(medio: String, id: i32) -> MedioPago {
        MedioPago { medio, id }
    }
}

impl PartialEq for MedioPago {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for MedioPago {}
