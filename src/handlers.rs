use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web::http::header::{self, Accept};
use serde::Serialize;
use mime;

// Define the struct for the JSON response
#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

// Modify the handler signature to accept HttpRequest
pub async fn hello(req: HttpRequest) -> impl Responder {
    // Check the Accept header
    let accept_header: Option<Accept> = req.headers().get(header::ACCEPT)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.parse().ok());

    // Determine the preferred content type
    let json_preferred = accept_header.map_or(false, |accept| {
        accept.iter().any(|pref| {
            let mime = pref.item.as_ref();
            mime.type_() == mime::APPLICATION && mime.subtype() == mime::JSON
        })
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