use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Field-level validation error detail.
#[derive(Debug, serde::Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

/// Application error types that map to HTTP responses.
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation failed")]
    Validation(Vec<FieldError>),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                serde_json::json!({"error": self.to_string()}),
            ),
            Self::Conflict(msg) => (StatusCode::CONFLICT, serde_json::json!({"error": msg})),
            Self::Validation(errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                serde_json::json!({"error": "Validation failed", "fields": errors}),
            ),
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                serde_json::json!({"error": self.to_string()}),
            ),
            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                serde_json::json!({"error": self.to_string()}),
            ),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, serde_json::json!({"error": msg})),
            Self::Internal(err) => {
                tracing::error!(?err, "Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    serde_json::json!({"error": "Internal server error"}),
                )
            }
        };

        (status, Json(body)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Internal(other.into()),
        }
    }
}
