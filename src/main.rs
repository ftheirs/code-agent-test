use actix_web::{App, HttpServer};
use env_logger;
use log::info;

mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_config = match config::AppConfig::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!("Failed to load configuration: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to load configuration",
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