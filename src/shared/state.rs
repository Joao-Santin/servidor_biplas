use std::sync::{Arc, atomic::AtomicI32};
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::Mutex;
use crate::infrastructure::services::egestor::client::EgestorClient;
use crate::config::settings::Settings;
use std::str::FromStr;

pub enum SectorType{
    Injection,
    ToolShop,
    Mill,
    Refrigeration,
    Assembly
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
pub enum ControllerType{
    Esp32,
    Plc,
    Arduino,
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

pub type Machines = Arc<Mutex<HashMap<String, MachineState>>>;

#[derive(Clone)]
pub struct AppState{
    pub settings: Settings,
    pub egestor: Arc<EgestorClient>,
    pub contador: Arc<AtomicI32>,
    pub machines: Machines
}

impl AppState{
    pub async fn new()-> Self{
        let settings = Settings::new().expect("Erro ao carregar settings");
        let egestor = EgestorClient::new(
            settings.egestor_personal_token.clone(),
        )
            .await
            .expect("Erro ao autenticar no eGestor");

        Self {
            settings,
            egestor: Arc::new(egestor),
            contador: Arc::new(AtomicI32::new(0)),
            machines: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}
