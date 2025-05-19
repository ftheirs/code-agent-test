se actix_web::{App, HttpServer, Responder, Route, web};
use dotenv::dotenv;
use env_logger;

mod config;
mod routes;
mod handlers;
mod errors;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
