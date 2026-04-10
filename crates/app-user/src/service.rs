use std::sync::Arc;

use uuid::Uuid;

use app_core::error::AppError;
use app_core::pagination::{OffsetPagination, PaginationParams};

use crate::dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
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
        params: PaginationParams,
    ) -> Result<(Vec<UserResponse>, OffsetPagination), AppError> {
        let params = params.clamp();
        let total_items = self.repo.count_all().await?;
        let users = self.repo.find_all(params.size, params.offset()).await?;
        let pagination = OffsetPagination::new(total_items, params.page, params.size);
        Ok((
            users.into_iter().map(UserResponse::from).collect(),
            pagination,
        ))
    }

    pub async fn get_user(&self, id: Uuid) -> Result<UserResponse, AppError> {
        let user = self.repo.find_by_id(id).await?.ok_or(AppError::NotFound)?;
        Ok(UserResponse::from(user))
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> Result<UserResponse, AppError> {
        if self.repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("email already exists".into()));
        }
        let user = self.repo.create(&req.name, &req.email).await?;
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
            .ok_or(AppError::NotFound)?;
        Ok(UserResponse::from(user))
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        if !self.repo.delete(id).await? {
            return Err(AppError::NotFound);
        }
        Ok(())
    }
}
