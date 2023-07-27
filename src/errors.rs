use actix_web::{
    error::{self, BlockingError},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use anyhow::anyhow;
use log::{error, warn};
use serde_json::json;
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("An internal error occurred. Please try again later.")]
    InternalError(anyhow::Error),
    #[error("Entity not found")]
    NotFound,
    #[error("Conflict: {0}")]
    ConflictInRequest(String),
}

impl ApiError {
    pub fn new_internal(error: anyhow::Error) -> Self {
        error!("Internal error: {error:?}");
        ApiError::InternalError(error)
    }

    fn new_conflict(error: impl Display) -> Self {
        let error = error.to_string();
        warn!("Conflict: {error}");
        ApiError::ConflictInRequest(error)
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
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::ConflictInRequest(_) => StatusCode::CONFLICT,
            ApiError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<r2d2::Error> for ApiError {
    fn from(error: r2d2::Error) -> ApiError {
        let error = anyhow::Error::new(error).context("r2d2 error");
        ApiError::new_internal(error)
    }
}

impl From<BlockingError> for ApiError {
    fn from(error: BlockingError) -> Self {
        let error = anyhow!(error).context("Blocking error");
        ApiError::new_internal(error)
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::InvalidCString(_) => todo!(),
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                err,
            ) => Self::new_conflict(err.message()),
            diesel::result::Error::NotFound => Self::NotFound,
            err => {
                let err = anyhow!(err).context("diesel error");
                Self::new_internal(err)
            }
        }
    }
}
