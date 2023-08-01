use actix_web::{
    error::{self, BlockingError},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use anyhow::anyhow;
use log::{error, warn};
use repository::errors::RepositoryError;
use serde_json::json;
use std::fmt::Debug;
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
    pub fn new_validation(error: impl Display + Debug) -> Self {
        warn!("Validation error: {error:?}");
        ApiError::ValidationError(error.to_string())
    }
    pub fn new_internal(error: anyhow::Error) -> Self {
        error!("Internal error: {error:?}");
        ApiError::InternalError(error)
    }

    fn new_conflict(error: impl Display + Debug) -> Self {
        let error = error.to_string();
        warn!("Conflict: {error:?}");
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

impl From<RepositoryError> for ApiError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::ValidationError(err) => Self::new_validation(err),
            RepositoryError::Conflict(err) => Self::new_conflict(err),
            RepositoryError::InternalError(err) => Self::new_internal(err),
            RepositoryError::NotFound => Self::NotFound,
        }
    }
}

impl From<BlockingError> for ApiError {
    fn from(error: BlockingError) -> Self {
        let error = anyhow!(error).context("Blocking error");
        ApiError::new_internal(error)
    }
}
