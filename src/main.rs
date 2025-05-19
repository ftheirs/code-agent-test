use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);

    println!("Starting server at http://{}", address);

    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes)
    })
    .bind(&address)?
    .run()
    .await
}
