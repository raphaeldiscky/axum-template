use std::sync::Arc;

use uuid::Uuid;

use app_core::error::AppError;
use app_core::pagination::{CursorData, CursorPagination, CursorParams};

use crate::dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::error::UserError;
use crate::repository::UserRepository;

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_users(
        &self,
        params: CursorParams,
    ) -> Result<(Vec<UserResponse>, CursorPagination), AppError> {
        let params = params.clamp();
        let cursor = params
            .cursor
            .as_deref()
            .and_then(CursorData::decode)
            .map(|c| (c.created_at, c.id));

        // Fetch limit + 1 to detect if there are more results.
        let mut users = self.repo.find_all(params.limit + 1, cursor).await?;

        let pagination =
            CursorPagination::from_results(&mut users, params.limit, |user| CursorData {
                id: user.id,
                created_at: user.created_at,
            });

        Ok((
            users.into_iter().map(UserResponse::from).collect(),
            pagination,
        ))
    }

    pub async fn get_user(&self, id: Uuid) -> Result<UserResponse, AppError> {
        let user = self.repo.find_by_id(id).await?.ok_or(UserError::NotFound)?;
        Ok(UserResponse::from(user))
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> Result<UserResponse, AppError> {
        if self.repo.find_by_email(&req.email).await?.is_some() {
            return Err(UserError::EmailConflict(req.email).into());
        }
        let user = self.repo.create(&req.name, &req.email).await?;
        tracing::info!(user_id = %user.id, email = %user.email, "user created");
        Ok(UserResponse::from(user))
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        req: UpdateUserRequest,
    ) -> Result<UserResponse, AppError> {
        let user = self
            .repo
            .update(id, req.name.as_deref(), req.email.as_deref())
            .await?
            .ok_or(UserError::NotFound)?;
        tracing::info!(user_id = %user.id, "user updated");
        Ok(UserResponse::from(user))
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        if !self.repo.delete(id).await? {
            return Err(UserError::NotFound.into());
        }
        tracing::info!(user_id = %id, "user deleted");
        Ok(())
    }
}
