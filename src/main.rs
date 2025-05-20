use actix_web::{App, HttpServer};
use env_logger;
use log::info;
use dotenv::dotenv;

mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if it exists. This should be done before loading config.
    dotenv().ok();

    // Initialize logger - env_logger reads RUST_LOG environment variable
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_config = match config::AppConfig::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!("Failed to load configuration: {}", e);
            // Exit the application if configuration loading fails
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to load configuration: {}", e),
            ));
        }
    };

    let bind_address = format!("{}:{}", app_config.host, app_config.port);
    info!("Starting Actix Web server on {}", bind_address);

    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}