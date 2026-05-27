use axum::{
    extract::State,
    Json,
};

// use serde_json::Value;
use crate::{
    domain::egestor::contato::{
        entity::ContatoCriadoResponse,
    },
    shared::state::AppState,
};
use crate::interfaces::http::dto::egestor_criar_contato::CriarContatoDTO;
use crate::{domain::egestor::contato::entity::ContatosResponse};

pub async fn listar_contatos(
    State(state): State<AppState>,
) -> Json<ContatosResponse> {

    let client = state.egestor;
    let contatos = client.listar_contatos().await.unwrap();
    Json(contatos)
}
pub async fn criar_contato(
    State(state): State<AppState>,
    Json(payload): Json<CriarContatoDTO>,
) -> Json<ContatoCriadoResponse> {

    let client = state.egestor;

    let contato = client
        .criar_contato(payload)
        .await
        .unwrap();

    Json(contato)
}
