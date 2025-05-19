use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn init() -> Result<Self, ConfigError> {
        let mut s = config::Config::default();

        s.merge(config::File::with_name("config/default"))?;

        s.merge(config::Environment::with_prefix("APP"))?;

        s.try_into()
    }
}