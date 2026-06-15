use crate::{infrastructure::services::bd_sql::machine_repository::MachineRepository, shared::state::AppState};

pub async fn start_machines_cleanup(
    state: AppState,
) {
    tokio::spawn(async move {
        loop {
            if let Err(err) =
                MachineRepository::update_online_status(
                    &state.database
                ).await
            {
                println!("Erro ao atualizar máquinas: {:?}", err);
            }

            tokio::time::sleep(
                tokio::time::Duration::from_secs(5)
            ).await;
        }
    });
}
