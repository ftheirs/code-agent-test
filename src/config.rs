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
            // Add configuration from a .env file (optional)
            .add_source(
                config::File::from(".env")
                    .format(config::FileFormat::DotEnv)
                    .required(false),
            )
            // Add configuration from environment variables (with a prefix, e.g., APP_)
            // Environment variables typically override file values by default precedence
            .add_source(config::Environment::default().separator("__")); // Use __ for nested config if needed

        builder.build()?.try_deserialize()
    }
}