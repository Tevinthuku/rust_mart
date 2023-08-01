use anyhow::anyhow;
use thiserror::Error;
use tokio::task::JoinError;

impl From<diesel::result::Error> for RepositoryError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                err,
            ) => Self::new_conflict(anyhow!(err.message().to_owned())),
            diesel::result::Error::NotFound => Self::NotFound,
            err => {
                let err = anyhow!(err).context("diesel error");
                Self::new_internal(err)
            }
        }
    }
}

impl From<JoinError> for RepositoryError {
    fn from(value: JoinError) -> Self {
        let err = anyhow!(value);
        RepositoryError::new_internal(err)
    }
}

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Validation error: {0}")]
    ValidationError(anyhow::Error),
    #[error("Conflict in resource: {0}")]
    Conflict(anyhow::Error),
    #[error("An internal error occurred. Please try again later.")]
    InternalError(anyhow::Error),
    #[error("Entity not found")]
    NotFound,
}

impl RepositoryError {
    pub fn new_validation(error: anyhow::Error) -> Self {
        Self::ValidationError(error)
    }

    pub fn new_internal(error: anyhow::Error) -> Self {
        Self::InternalError(error)
    }

    fn new_conflict(error: anyhow::Error) -> Self {
        Self::Conflict(error)
    }
}
