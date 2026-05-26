use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::domain::egestor::contato::entity::{ContatoCriadoResponse, ContatosResponse};

use crate::domain::egestor::contato::{
    dto::CriarContatoDTO,
};

#[derive(Clone)]
pub struct EgestorClient {
    pub client: Client,
    pub access_token: String,
}

#[derive(Serialize)]
struct LoginRequest {
    grant_type: String,
    personal_token: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    access_token: String,

}


impl EgestorClient {
    pub async fn new(
        personal_token: String,
    ) -> Result<Self, reqwest::Error> {

        let client = Client::new();

        let response = client
            .post("https://api.egestor.com.br/api/oauth/access_token")
            .json(&LoginRequest {
                grant_type: "personal".into(),
                personal_token,
            })
            .send()
            .await?;

        let data: LoginResponse = response.json().await?;

        Ok(Self {
            client,
            access_token: data.access_token,
        })
    }

    pub async fn listar_contatos(
        &self,
    ) -> Result<ContatosResponse, reqwest::Error> {

        let response = self.client
            .get("https://api.egestor.com.br/api/v1/contatos")
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        response.json::<ContatosResponse>().await
    }
    pub async fn criar_contato(
        &self,
        payload: CriarContatoDTO,
    ) -> Result<ContatoCriadoResponse, reqwest::Error> {

        let response = self.client
            .post("https://api.egestor.com.br/api/v1/contatos")
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await?;

        response
            .json::<ContatoCriadoResponse>()
            .await
    }
}
