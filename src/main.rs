use crate::interfaces::http::routes::create_router;
use crate::shared::state::AppState;
use crate::application::tasks::machines_cleanup::start_machines_cleanup;

mod shared;
mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

#[tokio::main]
async fn main() {
    let state = AppState::new().await;
    start_machines_cleanup(
        state.clone()
    ).await;
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(
        "0.0.0.0:3000"
    )
        .await
        .unwrap();

    println!("Servidor rodando!");

    axum::serve(listener, app)
        .await
        .unwrap()
}

