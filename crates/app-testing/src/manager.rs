use sqlx::PgPool;

use crate::postgres::{PostgresConfig, PostgresContainer};

/// Orchestrates test containers. Start all configured containers,
/// then access their connections via getter methods.
pub struct ContainerManager {
    pg_config: Option<PostgresConfig>,
    pg: Option<PostgresContainer>,
}

impl ContainerManager {
    pub const fn new() -> Self {
        Self {
            pg_config: None,
            pg: None,
        }
    }

    /// Configure a `PostgreSQL` container.
    #[must_use]
    pub fn with_postgres(mut self, config: PostgresConfig) -> Self {
        self.pg_config = Some(config);
        self
    }

    /// Start all configured containers.
    pub async fn start(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(config) = self.pg_config.take() {
            self.pg = Some(PostgresContainer::start(config).await?);
        }
        Ok(self)
    }

    /// Get the `PostgreSQL` connection pool.
    pub fn postgres_pool(&self) -> Option<&PgPool> {
        self.pg.as_ref().map(PostgresContainer::pool)
    }

    /// Truncate configured tables (keeps containers running).
    pub async fn cleanup_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(pg) = &self.pg {
            pg.cleanup_data().await?;
        }
        Ok(())
    }
}

impl Default for ContainerManager {
    fn default() -> Self {
        Self::new()
    }
}
