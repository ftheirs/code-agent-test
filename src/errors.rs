/ src/errors.rs

use actix_web::{ResponseError, HttpResponse};
use derive_more::{Display, From};

#[derive(Display, From, Debug)]
pub enum CustomError {
    NotFound,
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::NotFound => HttpResponse::NotFound().body("Not Found")
        }
    }
}
