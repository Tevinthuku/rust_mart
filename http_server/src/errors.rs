use log::error;
use serde_json::json;
use thiserror::Error;

use actix_web::{
    error::{self},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("An internal error occurred. Please try again later.")]
    InternalError(anyhow::Error),
}

impl ApiError {
    pub fn new_internal(err: anyhow::Error) -> Self {
        error!("Internal error: {err:?}");
        ApiError::InternalError(err)
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        ApiError::new_internal(value)
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(json!({
                "error": self.to_string()
            }))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
