// use std::collections::HashMap;
// use tokio::sync::Mutex;
// use std::env;
use std::sync::{Arc, atomic::AtomicI32};
use sqlx::PgPool;
use crate::infrastructure::services::egestor::client::EgestorClient;
use crate::config::settings::Settings;


#[derive(Clone)]
pub struct AppState{
    pub database: PgPool,
    pub settings: Settings,
    pub egestor: Arc<EgestorClient>,
    pub contador: Arc<AtomicI32>,
}

impl AppState{
    pub async fn new()-> Self{
        let settings = Settings::new().expect("Erro ao carregar settings");
        let pool = PgPool::connect(
            &settings.database_url.clone()
        ).await.expect("Erro ao autenticar SQL");
        let egestor = EgestorClient::new(
            settings.egestor_personal_token.clone(),
        )
            .await
            .expect("Erro ao autenticar no eGestor");

        Self {
            database: pool,
            settings,
            egestor: Arc::new(egestor),
            contador: Arc::new(AtomicI32::new(0)),
        }
    }
}
