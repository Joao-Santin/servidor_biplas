use std::env;

#[derive(Clone)]
pub struct Settings{
    pub database_url: String,
    pub egestor_personal_token: String,
}

impl Settings{
    pub fn new() -> Result<Self, env::VarError>{
        dotenv::dotenv().ok();
        Ok(Self{
            database_url: env::var("DATABASE_URL").unwrap(),
            egestor_personal_token: env::var("TOKENEGESTOR")?,
        })
    }

}
