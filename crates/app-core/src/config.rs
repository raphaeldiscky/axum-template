use figment::Figment;
use figment::providers::Env;
use serde::Deserialize;

/// Application configuration loaded from environment variables.
///
/// Required: `DATABASE_URL`, `JWT_SECRET`, `PORT`.
/// Optional: `HOST` (default: 0.0.0.0), `JWT_EXPIRATION_HOURS` (default: 24), `RUST_LOG` (default: info).
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_host")]
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    #[serde(default = "default_jwt_expiration_hours")]
    pub jwt_expiration_hours: u64,
    #[serde(default = "default_rust_log")]
    pub rust_log: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

const fn default_jwt_expiration_hours() -> u64 {
    24
}

fn default_rust_log() -> String {
    "info".to_string()
}

impl AppConfig {
    /// Load configuration from environment variables.
    /// Fails if required variables (`DATABASE_URL`, `JWT_SECRET`, `PORT`) are missing.
    pub fn load() -> Result<Self, Box<figment::Error>> {
        Figment::new().merge(Env::raw()).extract().map_err(Box::new)
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
