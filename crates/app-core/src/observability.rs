use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize the tracing subscriber with an environment filter.
///
/// The filter string follows `tracing_subscriber::EnvFilter` syntax (e.g., "info", "`app_core=debug`").
/// Falls back to the provided `default_filter` if `RUST_LOG` is not set.
pub fn init_tracing(default_filter: &str) {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| default_filter.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
