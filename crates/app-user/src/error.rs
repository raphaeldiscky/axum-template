use app_core::error::AppError;

/// Domain-specific errors for the user module.
#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound,

    #[error("Email already exists: {0}")]
    EmailConflict(String),
}

impl From<UserError> for AppError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::NotFound => Self::NotFound,
            UserError::EmailConflict(email) => {
                Self::Conflict(format!("email already exists: {email}"))
            }
        }
    }
}
