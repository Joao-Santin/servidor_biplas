use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub enum SectorType{
    Injection,
    ToolShop,
    Mill,
    Refrigeration,
    Assembly
}
impl Display for SectorType{
    fn fmt(
        &self,
        f: &mut Formatter<'_>
    )-> std::fmt::Result{
        let value = match self{
            Self::Injection => "INJECTION",
            Self::ToolShop => "TOOL_SHOP",
            Self::Mill => "MILL",
            Self::Refrigeration => "REFRIGERATION",
            Self::Assembly => "ASSEMBLY",
        };
        write!(f, "{}", value)
    }
}
impl FromStr for SectorType {

    type Err = String;

    fn from_str(
        value: &str,
    ) -> Result<Self, Self::Err> {

        match value {

            "INJECTION" => Ok(Self::Injection),

            "TOOL_SHOP" => Ok(Self::ToolShop),

            "MILL" => Ok(Self::Mill),

            "REFRIGERATION" => Ok(Self::Refrigeration),

            "ASSEMBLY" => Ok(Self::Assembly),

            _ => Err(
                format!(
                    "Setor inválido: {}",
                    value
                )
            )
        }
    }
}
