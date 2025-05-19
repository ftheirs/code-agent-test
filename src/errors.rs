use actix_web::{ResponseError, HttpResponse};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Deserialize, Serialize)]
pub enum CustomError {
    #[display(fmt = "{}")]
    InternalServerError,

    #[display(fmt = "{}")]
    BadRequest,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            CustomError::BadRequest => HttpResponse::BadRequest().json("Bad Request")
        }
    }
}