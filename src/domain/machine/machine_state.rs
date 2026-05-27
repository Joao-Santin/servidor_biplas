use crate::domain::machine::sector_type::SectorType;
use crate::domain::machine::controller_type::ControllerType;
use std::time::Instant;

pub struct MachineState{
    pub id: String,
    pub sector: SectorType,
    pub controller: ControllerType,
    pub ip: String,
    pub mac: String,
    pub timestamp: u64,
    pub last_seen: Instant,
}

impl MachineState{
    pub fn is_online(&self) -> bool{
        self.last_seen.elapsed().as_secs() < 15
    }
}
