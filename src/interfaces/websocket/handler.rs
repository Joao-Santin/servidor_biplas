use std::{str::FromStr, time::Instant};

use axum::{
    extract::{ws::{
        Message,
        WebSocket,
        WebSocketUpgrade,
    }, State},
    response::IntoResponse,
};
use crate::{interfaces::websocket::dto::esp_message::EspMessage, shared::state::{AppState, ControllerType, MachineState, SectorType}};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>
) -> impl IntoResponse {

    ws.on_upgrade(move |socket| {
        handle_socket(socket, state)

    })
}

async fn handle_socket(
    mut socket: WebSocket,
    state: AppState
) {
    while let Some(result) = socket.recv().await {

        match result {

            Ok(Message::Text(text)) => {
                let mut machines = state.machines.lock().await;
                    match serde_json::from_str::<EspMessage>(&text) {
                        Ok(message) =>{
                        match message{
                            EspMessage::Identify {id, sector, controller, ip, mac, timestamp, payload} => {
                                let controller: ControllerType = controller.parse().unwrap();
                                let sector: SectorType = sector.parse().unwrap();
                                let last_seen:Instant = Instant::now();
                                machines.insert(id.clone(), MachineState{
                                    id, sector, controller, ip, mac, timestamp, last_seen
                                });
                            }
                            EspMessage::Heartbeat {id, sector, controller, ip, mac, timestamp, payload} => {
                                let controller: ControllerType = controller.parse().unwrap();
                                let sector: SectorType = sector.parse().unwrap();
                                let last_seen:Instant = Instant::now();
                                machines.insert(id.clone(), MachineState{
                                    id, sector, controller, ip, mac, timestamp, last_seen
                                });
                            }
                            EspMessage::SignalReceived {..} => {
                                println!("SignalReceived:{:?}", message)
                            }
                        }
                    }
                        //
                        // Ok(data) => {
                        //     println!("Recebido: {:?}", data);
                        // }
                        //
                        Err(_) => {
                            println!("Mensagem simples: {}", text);
                        }
                    }

                    let response =
                        Message::Text(
                            format!("Servidor recebeu: {}", text).into()
                        );

                    if let Err(err) = socket.send(response).await {

                        println!("Erro ao enviar: {:?}", err);

                        break;
                    }
            }

            Ok(Message::Close(_)) => {

                println!("Cliente fechou conexão");

                break;
            }

            Ok(Message::Ping(_)) => {

                println!("Ping recebido");
            }

            Ok(Message::Pong(_)) => {

                println!("Pong recebido");
            }

            Err(err) => {

                println!("Erro websocket: {:?}", err);

                break;
            }

            _ => {}
        }
    }

    println!("Cliente desconectado!");
}
