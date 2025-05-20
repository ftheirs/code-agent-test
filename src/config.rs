use serde::Deserialize;
use config::ConfigError;

#[derive(Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let builder = config::Config::builder()
            .add_source(config::Environment::default());

        // Optional: Add .env file support in development
        #[cfg(debug_assertions)]
        let builder = builder.add_source(
            config::File::from(".env").required(false)
        );

        builder.build()?.try_deserialize()
    }
}