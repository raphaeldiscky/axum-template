use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use app_core::error::AppError;
use app_core::validation::ValidatedJson;

use crate::dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::service::UserService;

pub async fn list(State(service): State<UserService>) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = service.list_users().await?;
    Ok(Json(users))
}

pub async fn create(
    State(service): State<UserService>,
    ValidatedJson(input): ValidatedJson<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = service.create_user(input).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = service.get_user(id).await?;
    Ok(Json(user))
}

pub async fn update(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
    ValidatedJson(input): ValidatedJson<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = service.update_user(id, input).await?;
    Ok(Json(user))
}

pub async fn delete(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
