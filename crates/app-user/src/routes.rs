use axum::Router;
use axum::extract::FromRef;
use axum::routing::get;

use crate::handler;
use crate::service::UserService;

/// Build the user routes. The generic `S` allows this router to be nested
/// into any `AppState` that implements `FromRef<S>` for `UserService`.
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    UserService: FromRef<S>,
{
    Router::new()
        .route("/", get(handler::list).post(handler::create))
        .route(
            "/{id}",
            get(handler::get)
                .put(handler::update)
                .delete(handler::delete),
        )
}
