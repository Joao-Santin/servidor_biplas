pub enum MachineStatus{
    All,
    Online,
    Offline,
}
impl MachineStatus{
    pub fn by_option_bool(
        value: Option<bool>
    )->Self{
        match value{
            None => Self::All,
            Some(true) => Self::Online,
            Some(false) => Self::Offline,
        }
    }
}
