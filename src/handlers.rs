use actix_web::http::header::{self};
use actix_web::{HttpRequest, HttpResponse, Responder};
use mime::{Mime, APPLICATION, JSON};
use serde::Serialize;

// Define the struct for the JSON response
#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

// Modify the handler signature to accept HttpRequest
pub async fn hello(req: HttpRequest) -> impl Responder {
    // Check the Accept header
    let accept_header = req
        .headers()
        .get(header::ACCEPT)
        .and_then(|hv| hv.to_str().ok());

    // Determine the preferred content type
    let json_preferred = accept_header.map_or(false, |accept| {
        accept
            .split(',')
            .filter_map(|s| s.trim().parse::<Mime>().ok())
            .any(|mime| mime.type_() == APPLICATION && mime.subtype() == JSON)
    });

    if json_preferred {
        // Create and serialize the JSON response
        let response_data = HelloResponse {
            message: "Hello World!".to_string(),
        };
        // Use .json() shortcut which sets Content-Type to application/json
        HttpResponse::Ok().json(response_data)
    } else {
        // Return plain text response
        HttpResponse::Ok().body("Hello World!")
    }
}
