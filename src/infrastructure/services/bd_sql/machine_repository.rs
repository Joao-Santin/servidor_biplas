use sqlx::{PgPool, Row};
use std::time::Instant;
use crate::domain::machine::machine_state::MachineState;
use crate::domain::machine::machine_status::MachineStatus;

pub struct MachineRepository;
impl MachineRepository {

    pub async fn upsert_machine(
        pool: &PgPool,
        machine: &MachineState,
    ) -> Result<(), sqlx::Error> {

        sqlx::query(
            "
            INSERT INTO machines
            (
                id,
                sector,
                controller,
                ip,
                mac,
                online,
                last_timestamp,
                last_heartbeat
            )

            VALUES

            (
                $1,$2,$3,$4,$5,true,$6,NOW()
            )

            ON CONFLICT(id)

            DO UPDATE SET

                sector=$2,
                controller=$3,
                ip=$4,
                mac=$5,
                online=true,
                last_timestamp=$6,
                last_heartbeat=NOW()
            "
        )

        .bind(&machine.id)
        .bind(machine.sector.to_string())
        .bind(machine.controller.to_string())
        .bind(&machine.ip)
        .bind(&machine.mac)
        .bind(machine.timestamp)

        .execute(pool)

        .await?;

        Ok(())
    }
    pub async fn find(
        pool: &PgPool,
        filter: MachineStatus,
    ) -> Result<Vec<MachineState>, sqlx::Error> {

        let query = match filter {

            MachineStatus::All =>
                "SELECT * FROM machines",

            MachineStatus::Online =>
                "
                SELECT *
                FROM machines
                WHERE online = true
                ",

            MachineStatus::Offline =>
                "
                SELECT *
                FROM machines
                WHERE online = false
                ",
        };

        let rows =
            sqlx::query(query)
            .fetch_all(pool)
            .await?;

        // map para MachineState...
        let machines = rows
            .into_iter()
            .map(|row| {

                MachineState {
                    id: row.get("id"),
                    sector: row.get::<String,_>("sector").parse().unwrap(),
                    controller: row.get::<String,_>("controller").parse().unwrap(),
                    ip: row.get("ip"),
                    mac: row.get("mac"),
                    timestamp: row.get::<i64,_>("last_timestamp"),
                    last_seen: Instant::now(),
                }

            })
            .collect();

        Ok(machines)
    }
}
