use axum::{Json, http::StatusCode, response::ErrorResponse};
use thiserror::Error;

use crate::{api::ErrorResponse as ApiErrorResponse, log};

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

    #[error("No migrations have been applied to this environment")]
    NoMigrations,

    #[error("The NocoDB instance is not available; it may be overloaded or still starting up.")]
    NocoUnavailable,

    #[error("The environment is missing required configuration.")]
    MissingEnvConfig,

    #[error("The requested asset was not found.")]
    AssetNotFound,

    #[error("Invalid custom domain: {0}")]
    InvalidDomain(anyhow::Error),

    #[error("That custom domain is already assigned to a different environment.")]
    DomainInUse,

    #[error("No custom domain is configured for this environment.")]
    NoEnvDomain,

    #[error("The submitted schedule is invalid.")]
    InvalidSchedule,

    #[error("No synced schedule exists for that sync code.")]
    ScheduleNotFound,

    #[error("A valid password is required to access this app.")]
    Unauthorized,

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
            Error::NoMigrations => StatusCode::NOT_FOUND,
            Error::NocoUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            Error::MissingEnvConfig => StatusCode::INTERNAL_SERVER_ERROR,
            Error::AssetNotFound => StatusCode::NOT_FOUND,
            Error::InvalidDomain(_) => StatusCode::BAD_REQUEST,
            Error::DomainInUse => StatusCode::CONFLICT,
            Error::NoEnvDomain => StatusCode::NOT_FOUND,
            Error::InvalidSchedule => StatusCode::BAD_REQUEST,
            Error::ScheduleNotFound => StatusCode::NOT_FOUND,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<Error> for ErrorResponse {
    fn from(error: Error) -> Self {
        log::error(log::Message {
            id: "admin-error",
            env: log::EnvRef::Global,
            msg: &error.to_string(),
        });

        ErrorResponse::from((
            error.status_code(),
            Json(ApiErrorResponse {
                error: error.to_string(),
            }),
        ))
    }
}
