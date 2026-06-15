use crate::domain::machine::sector_type::SectorType;
use crate::domain::machine::controller_type::ControllerType;

pub struct MachineState{
    pub id: String,
    pub sector: SectorType,
    pub controller: ControllerType,
    pub ip: String,
    pub mac: String,
    pub timestamp: i64,
    pub online: bool,
}

