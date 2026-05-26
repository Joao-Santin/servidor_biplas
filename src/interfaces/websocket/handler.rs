use axum::{
    extract::ws::{
        Message,
        WebSocket,
        WebSocketUpgrade,
    },
    response::IntoResponse,
};

//assim que tem que ser
//retirar trecho abaixo até onde tiver segundo comentario
use serde::Deserialize;//tirar isso daqui depois
#[derive(Debug, Deserialize)]
struct EspMessage{
    machine_id: Option<String>,
    event: Option<String>,
}
//retirar trecho acima até onde tiver segundo comentario
//
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(
    mut socket: WebSocket,
) {

    println!("Cliente conectado!");

    while let Some(result) = socket.recv().await {

        match result {

            Ok(Message::Text(text)) => {
                println!("Recebido bruto: {}", text);

                    match serde_json::from_str::<EspMessage>(&text) {

                        Ok(data) => {

                            println!("Machine ID: {:?}", data.machine_id);

                            println!("Event: {:?}", data.event);

                        }

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
