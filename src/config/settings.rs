use std::env;

#[derive(Clone)]
pub struct Settings{
    pub egestor_personal_token: String,
}

impl Settings{
    pub fn new() -> Result<Self, env::VarError>{
        dotenv::dotenv().ok();
        Ok(Self{
            egestor_personal_token: env::var("TOKENEGESTOR")?,
        })
    }

}
