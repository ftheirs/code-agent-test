use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok(); // Load .env file if it exists

        let mut settings = Config::new();

        // Add configuration from environment variables
        // E.g., APP__HOST, APP__PORT, APP__LOG_LEVEL
        settings.merge(Environment::with_prefix("APP").separator("__"))?;

        // Add default values if not set by environment variables
        // These defaults should align with techContext.md if specified there.
        // Assuming defaults: host="127.0.0.1", port=8080, log_level="info"
        settings.set_default("host", "127.0.0.1")?;
        settings.set_default("port", 8080)?;
        settings.set_default("log_level", "info")?;


        settings.try_into()
    }
}