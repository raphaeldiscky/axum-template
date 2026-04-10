use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use app_core::error::AppError;
use app_core::pagination::CursorParams;
use app_core::response::ApiResponse;
use app_core::validation::ValidatedJson;

use crate::dto::{CreateUserRequest, UpdateUserRequest};
use crate::service::UserService;

pub async fn list(
    State(service): State<UserService>,
    Query(params): Query<CursorParams>,
) -> Result<impl IntoResponse, AppError> {
    let (users, pagination) = service.list_users(params).await?;
    Ok(ApiResponse::with_pagination(users, pagination))
}

pub async fn create(
    State(service): State<UserService>,
    ValidatedJson(input): ValidatedJson<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = service.create_user(input).await?;
    Ok(ApiResponse::created(user))
}

pub async fn get(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = service.get_user(id).await?;
    Ok(ApiResponse::ok(user))
}

pub async fn update(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
    ValidatedJson(input): ValidatedJson<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = service.update_user(id, input).await?;
    Ok(ApiResponse::ok(user))
}

pub async fn delete(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
