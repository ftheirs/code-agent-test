use actix_web::{HttpResponse, Responder, web};
use crate::config::Config;

pub async fn index(app_config: web::Data<Config>) -> impl Responder {
    let host = &app_config.server.host;
    HttpResponse::Ok().body(format!("Hello from {}!", host))
}