use axum::Json;
use axum::extract::{FromRequest, Request, rejection::JsonRejection};
use validator::Validate;

use crate::error::{AppError, FieldError};

/// A JSON extractor that validates the payload using `validator` derives.
///
/// Usage in handlers:
/// ```ignore
/// async fn create(ValidatedJson(input): ValidatedJson<CreateRequest>) -> Result<..., AppError> { ... }
/// ```
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e: JsonRejection| AppError::BadRequest(e.body_text()))?;

        value.validate().map_err(|errors| {
            let field_errors = errors
                .field_errors()
                .into_iter()
                .flat_map(|(field, errs)| {
                    errs.iter().map(move |e| FieldError {
                        field: field.to_string(),
                        message: e
                            .message
                            .as_ref()
                            .map_or_else(|| e.code.to_string(), std::string::ToString::to_string),
                    })
                })
                .collect();
            AppError::Validation(field_errors)
        })?;

        Ok(Self(value))
    }
}
