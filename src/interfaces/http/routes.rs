use axum::{
    routing::{get, post},
    Router,
};

use crate::{interfaces::http::handlers::{contador::{decrement_contador, increment_contador}, egestor::{criar_contato, listar_contatos}, hello::helloo}, shared::state::AppState};
use crate::interfaces::websocket::handler::websocket_handler;
use crate::interfaces::http::handlers::machines::{get_machines, get_online_machines, get_offline_machines};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(helloo))
        .route("/egestor/listar_contatos", get(listar_contatos))
        .route("/egestor/contatos", post(criar_contato))
        .route("/contador/increment/{quantidade}", post(increment_contador))
        .route("/contador/decrement/{quantidade}", post(decrement_contador))
        .route("/machines", get(get_machines))
        .route("/machines/online", get(get_online_machines))
        .route("/machines/offline", get(get_offline_machines))
        .route("/ws", get(websocket_handler))
        .with_state(state)
}
