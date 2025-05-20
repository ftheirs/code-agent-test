use actix_web::{App, HttpResponse, HttpServer, Responder};
use api_rust::config::{self, AppConfig};
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::load().expect("Failed to load configuration");
    config::init_logger(&config.log_level);

    log::info!("Starting server at http://{}:{}/", config.host, config.port);

    HttpServer::new(|| {
        App::new()
            // Basic handler to ensure the server runs
            .default_service(actix_web::web::to(|| async {
                HttpResponse::NotFound().body("Not Found")
            }))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
