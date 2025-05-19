use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use log::info;
use env_logger;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server_port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let server_address = format!("127.0.0.1:{}", server_port);

    info!("Starting server at {}", server_address);

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(&server_address)?
    .run()
    .await
}
