use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Scylla {
    pub uri: String,
    pub keyspace: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Mqtt {
    pub client_id: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub scylla: Scylla,
    pub mqtt: Mqtt,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("SERVICE_PROFILE")
            .unwrap_or_else(|_| "development".into());

        let settings = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(Environment::with_prefix("refinery"))
            .build()?;

        settings.try_deserialize()
    }
}
