use crate::shared::state::AppState;
use axum::{extract::{State}, Json};
use crate::interfaces::http::dto::machine_response::MachineResponse;

pub async fn get_machines(State(state): State<AppState>)->Json<Vec<MachineResponse>>{
    let machines = state.machines.lock().await;
    let response: Vec<MachineResponse> = machines.values().map(MachineResponse::from).collect();
    Json(response)
}

pub async fn get_online_machines(
    State(state): State<AppState>
) -> Json<Vec<MachineResponse>> {

    let machines =
        state.machines.lock().await;

    let response: Vec<MachineResponse> =
        machines
            .values()
            .filter(|machine| machine.is_online())
            .map(MachineResponse::from)
            .collect();

    Json(response)
}

pub async fn get_offline_machines(
    State(state): State<AppState>
) -> Json<Vec<MachineResponse>> {

    let machines =
        state.machines.lock().await;

    let response: Vec<MachineResponse> =
        machines
            .values()
            .filter(|machine| !machine.is_online())
            .map(MachineResponse::from)
            .collect();

    Json(response)
}
