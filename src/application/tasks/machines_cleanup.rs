use crate::shared::state::AppState;

pub async fn start_machines_cleanup(
    state: AppState,
) {

    tokio::spawn(async move {

        loop {

            {
                let machines =
                    state
                        .machines
                        .lock()
                        .await;

                println!("======== MÁQUINAS ========");

                for (id, machine) in machines.iter() {

                    println!(
                        "{} -> {}",
                        id,
                        machine.is_online()
                    );
                }
            }

            tokio::time::sleep(
                tokio::time::Duration::from_secs(5)
            )
            .await;
        }
    });
}
