use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::extract::FromRef;
use axum::http::StatusCode;
use axum::routing::get;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use app_core::config::AppConfig;
use app_core::server;
use app_user::repository::PgUserRepository;
use app_user::service::UserService;

#[derive(Clone, FromRef)]
struct AppState {
    #[allow(dead_code)]
    config: AppConfig,
    pool: PgPool,
    user_service: UserService,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = AppConfig::load().expect("failed to load configuration");

    app_core::observability::init_tracing(&config.rust_log);

    let pool = app_core::db::create_pool(&config.database_url)
        .await
        .expect("failed to connect to database");

    let repo = PgUserRepository::new(pool.clone());
    let user_service = UserService::new(Arc::new(repo));

    let state = AppState {
        config: config.clone(),
        pool,
        user_service,
    };

    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive());

    let app = Router::new()
        .route("/healthz", get(server::liveness))
        .route("/readyz", get(server::readiness))
        .nest("/api/users", app_user::routes::router())
        .with_state(state)
        .layer(middleware);

    let listener = tokio::net::TcpListener::bind(config.addr())
        .await
        .expect("failed to bind address");

    tracing::info!("listening on {}", config.addr());

    axum::serve(listener, app)
        .with_graceful_shutdown(server::shutdown_signal())
        .await
        .expect("server error");
}
