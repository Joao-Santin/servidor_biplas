//handlers/hello
//
use serde::Serialize;
use axum::{Json};

#[derive(Serialize)]
pub struct HelloResponse{
    message:String,
}

pub async fn helloo() -> Json<HelloResponse>{
    Json(HelloResponse{
        message: "Olá, caro curioso! Como que está, tudo bem? Este é o primeiro route do servidor da Biplas. Tenha um ótimo dia! Torço para seu melhor. Aqui é onde João Vitor está tentando criar algo interessante e útil para toda a empresa.".into()
    })
}
