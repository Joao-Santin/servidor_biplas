use crate::{infrastructure::services::bd_sql::machine_repository::MachineRepository, interfaces::http::dto::machine_query::MachineQuery, shared::state::AppState};
use axum::{extract::{State, Query}, Json};
use crate::interfaces::http::dto::machine_response::MachineResponse;
use crate::domain::machine::machine_status::MachineStatus;

pub async fn get_machines(State(state): State<AppState>, Query(query): Query<MachineQuery>)->Json<Vec<MachineResponse>>{
    let status = MachineStatus::by_option_bool(
        query.online
    );
    match MachineRepository::find(&state.database, status).await{
        Ok(machines) => {
            let response: Vec<MachineResponse> = machines.iter().map(MachineResponse::from).collect();
            Json(response)
        }
        Err(err)=>{
            println!("{:?}", err);
            Json(Vec::new())
        }
    }
}

