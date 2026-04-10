use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::pagination::OffsetPagination;

/// Consistent API response envelope.
///
/// ```json
/// { "message": "Success", "data": { ... }, "pagination": { ... } }
/// ```
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<OffsetPagination>,
    #[serde(skip)]
    status: u16,
}

impl<T: Serialize> ApiResponse<T> {
    /// 200 OK with data.
    pub fn ok(data: T) -> Self {
        Self {
            message: "Success".to_string(),
            data,
            pagination: None,
            status: 200,
        }
    }

    /// 201 Created with data.
    pub fn created(data: T) -> Self {
        Self {
            message: "Created".to_string(),
            data,
            pagination: None,
            status: 201,
        }
    }

    /// 200 OK with data and pagination metadata.
    pub fn with_pagination(data: T, pagination: OffsetPagination) -> Self {
        Self {
            message: "Success".to_string(),
            data,
            pagination: Some(pagination),
            status: 200,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}
