use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub enum ControllerType{
    Esp32,
    Plc,
    Arduino,
}
impl Display for ControllerType{
    fn fmt(
        &self,
        f: &mut Formatter<'_>
    )-> std::fmt::Result{
        let value = match self{
            Self::Esp32 => "ESP32",
            Self::Plc => "PLC",
            Self::Arduino => "ARDUINO",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for ControllerType {

    type Err = String;

    fn from_str(
        value: &str,
    ) -> Result<Self, Self::Err> {

        match value {

            "ESP32" => Ok(Self::Esp32),

            "PLC" => Ok(Self::Plc),

            "ARDUINO" => Ok(Self::Arduino),

            _ => Err(
                format!(
                    "Controller inválido: {}",
                    value
                )
            )
        }
    }
}
