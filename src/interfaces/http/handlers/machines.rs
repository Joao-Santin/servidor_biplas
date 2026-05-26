use crate::shared::state::AppState;
use axum::{extract::{Path, State}, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct MachineResponse {

    pub id: String,

    pub online: bool,

    pub ip: String,

    pub mac: String,
}

pub async fn get_machines(State(state): State<AppState>)->Json<Vec<MachineResponse>>{
    let machines = state.machines.lock().await;
    let response: Vec<MachineResponse> = machines.values().map(
        |machine|{
            MachineResponse{
                id: machine.id.clone(),
                online: machine.is_online(),
                ip: machine.ip.clone(),
                mac: machine.mac.clone()
            }
        }
    ).collect();
    Json(response)
}
