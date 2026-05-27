use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CriarContatoDTO {
    pub nome: String,
    pub tipo: Vec<String>,

    pub fantasia: Option<String>,
    pub nomeParaContato: Option<String>,
    pub cpfcnpj: Option<String>,
    pub dtNasc: Option<String>,

    pub emails: Option<Vec<String>>,
    pub fones: Option<Vec<String>>,

    pub cep: Option<String>,
    pub logradouro: Option<String>,
    pub numero: Option<String>,
    pub complemento: Option<String>,
    pub bairro: Option<String>,

    pub codIBGE: Option<String>,
    pub uf: Option<String>,

    pub clienteFinal: Option<bool>,

    pub indicadorIE: Option<i32>,

    pub inscricaoMunicipal: Option<String>,
    pub inscricaoEstadual: Option<String>,

    pub obs: Option<String>,
    pub tags: Option<Vec<String>>,
}
