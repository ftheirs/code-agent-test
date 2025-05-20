use serde::Deserialize;
use config::ConfigError;

#[derive(Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

impl AppConfig {
    // Function to load configuration from environment variables and .env file
    pub fn load_config() -> Result<Self, ConfigError> {
        let builder = config::Config::builder()
            // The dotenv crate is used to load .env files. Configuration is then read from the environment.
            // Add configuration from environment variables (with a prefix, e.g., APP_)
            // Environment variables typically override file values by default precedence
            .add_source(config::Environment::default().separator("__")); // Use __ for nested config if needed

        builder.build()?.try_deserialize()
    }
}