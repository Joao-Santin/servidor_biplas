use serde::Deserialize;
#[derive(Deserialize)]
pub struct MachineQuery{
    pub online: Option<bool>,
}
