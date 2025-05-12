use axum::{Json, response::ErrorResponse};
use reqwest::StatusCode;
use worker::console_error;

use crate::api::ErrorResponse as ApiErrorResponse;

fn error_response(code: StatusCode, message: &str) -> ErrorResponse {
    console_error!("Error: {}", message);
    ErrorResponse::from((
        code,
        Json(ApiErrorResponse {
            error: message.to_string(),
        }),
    ))
}

pub fn err_no_api_token() -> ErrorResponse {
    error_response(
        StatusCode::CONFLICT,
        "There is no NocoDB API token configured for this environment.",
    )
}

pub fn err_no_env_id() -> ErrorResponse {
    error_response(
        StatusCode::NOT_FOUND,
        "You have not generated an app link for this environment.",
    )
}

pub fn err_no_base_id() -> ErrorResponse {
    error_response(
        StatusCode::NOT_FOUND,
        "A NocoDB base does not exist for this environment.",
    )
}

pub fn err_base_already_exists() -> ErrorResponse {
    error_response(
        StatusCode::CONFLICT,
        "A NocoDB base already exists for this environment.",
    )
}
