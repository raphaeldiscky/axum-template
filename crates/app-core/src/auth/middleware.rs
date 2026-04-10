use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

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
///     .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));
/// ```
pub async fn auth_middleware(request: Request, next: Next) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    // Extract JWT secret from request extensions (set by the state layer).
    let jwt_secret = request
        .extensions()
        .get::<JwtSecret>()
        .ok_or(AppError::Unauthorized)?;

    let claims = jwt::decode(&jwt_secret.0, token).map_err(|_| AppError::Unauthorized)?;

    let mut request = request;
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Wrapper type to store JWT secret in request extensions.
#[derive(Clone)]
pub struct JwtSecret(pub String);
