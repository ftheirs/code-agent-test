use actix_web::{get, HttpRequest};
use crate::handlers::hello_world_handler;

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index);
    cfg.service(hello);
}

#[get("/")]
async fn index(req: HttpRequest) -> impl actix_web::Responder {
    hello_world_handler(req).await
}

#[get("/hello")]
async fn hello(req: HttpRequest) -> impl actix_web::Responder {
    hello_world_handler(req).await
}
