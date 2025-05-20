use actix_web::{App, HttpServer};
use env_logger;
use log::info;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting Actix Web server");

    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}