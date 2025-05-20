use serde::Deserialize;
use config::{Config, ConfigError, Environment};
use std::env;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .set_default("host", "127.0.0.1")?
            .set_default("port", 8080)?
            .set_default("log_level", "info")?
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?;

        config.try_deserialize()
    }
}

pub fn init_logger(log_level: &str) {
    env::set_var("RUST_LOG", log_level);
    env_logger::init();
}