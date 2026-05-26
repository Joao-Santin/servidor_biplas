use serde::{Deserialize, Serialize};
use crate::shared::extensions::serde::string_or_number::deserialize_string_or_number;

#[derive(Deserialize, Serialize)]
pub struct ContatosResponse {
    pub total: i32,
    pub data: Vec<Contato>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contato {
    #[serde(deserialize_with="deserialize_string_or_number")]
    pub codigo: String,
    pub nome: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContatoCriadoResponse {
    pub codigo: i32,
    pub nome: String,
}
