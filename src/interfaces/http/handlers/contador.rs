use serde::Serialize;
use axum::{extract::{Path, State}, Json};
use crate::shared::state::AppState;

//testing
#[derive(Serialize)]
pub struct ContadorResponse{
    message:i32,
}
pub async fn increment_contador(State(state): State<AppState>, Path(quantidade): Path<i32>)-> Json<ContadorResponse>{
    state.contador.fetch_add(quantidade, std::sync::atomic::Ordering::SeqCst);
    Json(ContadorResponse { message: state.contador.load(std::sync::atomic::Ordering::SeqCst) })
}
pub async fn decrement_contador(State(state): State<AppState>, Path(quantidade): Path<i32>)-> Json<ContadorResponse>{
    state.contador.fetch_sub(quantidade, std::sync::atomic::Ordering::SeqCst);
    Json(ContadorResponse { message: state.contador.load(std::sync::atomic::Ordering::SeqCst) })
}
pub async fn read_contador(State(state): State<AppState>)->Json<ContadorResponse>{
    Json(ContadorResponse{
        message:state.contador.load(std::sync::atomic::Ordering::SeqCst)
    })
}
