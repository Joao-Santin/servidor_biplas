use serde::Serialize;

use crate::domain::machine::machine_state::MachineState;

#[derive(Serialize)]
pub struct MachineResponse {
    pub id: String,
    pub sector: String,
    pub controller: String,
    pub online: bool,
    pub timestamp: u64,
    pub ip: String,
    pub mac: String,
}
impl From<&MachineState> for MachineResponse{
    fn from(
        machine: &MachineState
    ) -> Self{
        Self{
            id: machine.id.clone(),
            sector: machine.sector.to_string(),
            controller: machine.controller.to_string(),
            online: machine.is_online(),
            timestamp: machine.timestamp.clone(),
            ip: machine.ip.clone(),
            mac: machine.mac.clone()
        }
    }
}
