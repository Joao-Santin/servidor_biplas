use std::sync::{Arc, atomic::AtomicI32};
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::Mutex;
use crate::infrastructure::services::egestor::client::EgestorClient;
use crate::config::settings::Settings;
//teste de novo state
#[derive(Clone)]
pub struct AppState{
    pub settings: Settings,
    pub egestor: Arc<EgestorClient>,
    pub contador: Arc<AtomicI32>
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
        }
    }
}
