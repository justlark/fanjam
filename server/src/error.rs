use axum::{Json, http::StatusCode, response::ErrorResponse};
use thiserror::Error;
use worker::console_error;

use crate::api::ErrorResponse as ApiErrorResponse;

#[derive(Debug, Error)]
pub enum Error {
    #[error("There is no NocoDB API token configured for this environment.")]
    NoApiToken,

    #[error("You have not generated an app link for this environment.")]
    NoEnvId,

    #[error("A NocoDB base does not exist for this environment.")]
    NoBaseId,

    #[error("A NocoDB base already exists for this environment.")]
    BaseAlreadyExists,

    #[error("You must specify the migration to roll back to.")]
    MissingMigrationVersion,

    #[error("The NocoDB instance is not available; it may still be starting up.")]
    NocoUnavailable,

    #[error("Internal server error: {0}")]
    Internal(anyhow::Error),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::NoApiToken => StatusCode::CONFLICT,
            Error::NoEnvId => StatusCode::NOT_FOUND,
            Error::NoBaseId => StatusCode::NOT_FOUND,
            Error::BaseAlreadyExists => StatusCode::CONFLICT,
            Error::MissingMigrationVersion => StatusCode::BAD_REQUEST,
            Error::NocoUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<Error> for ErrorResponse {
    fn from(error: Error) -> Self {
        console_error!("Error: {}", error);

        ErrorResponse::from((
            error.status_code(),
            Json(ApiErrorResponse {
                error: error.to_string(),
            }),
        ))
    }
}
