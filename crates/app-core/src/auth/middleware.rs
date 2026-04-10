use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;

use crate::auth::jwt;
use crate::error::AppError;

/// Axum middleware that validates JWT from the `Authorization: Bearer <token>` header.
///
/// On success, inserts [`jwt::Claims`] as a request extension so handlers can access it
/// via `Extension<Claims>`.
///
/// Usage:
/// ```ignore
/// use axum::middleware;
/// let protected = Router::new()
///     .route("/me", get(me_handler))
///     .layer(middleware::from_fn(auth_middleware));
/// ```
pub async fn auth_middleware(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let jwt_secret = request
        .extensions()
        .get::<JwtSecret>()
        .ok_or(AppError::Unauthorized)?;

    let claims = jwt::decode(&jwt_secret.0, auth.token()).map_err(|_| AppError::Unauthorized)?;

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Wrapper type to store JWT secret in request extensions.
#[derive(Clone)]
pub struct JwtSecret(pub String);
