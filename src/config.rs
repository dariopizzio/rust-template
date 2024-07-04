use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();
        envy::from_env::<Config>().expect("Invalid configuration file")
    }
}
