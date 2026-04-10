use std::path::Path;
use std::time::Duration;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use testcontainers::ContainerAsync;
use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

/// Configuration for a test `PostgreSQL` container.
pub struct PostgresConfig {
    pub database: String,
    pub username: String,
    pub password: String,
    pub image_tag: String,
    pub migration_paths: Vec<String>,
    pub cleanup_tables: Vec<String>,
}

impl PostgresConfig {
    pub fn new(database: &str) -> Self {
        Self {
            database: database.to_string(),
            username: "testuser".to_string(),
            password: "testpass".to_string(),
            image_tag: "18-alpine".to_string(),
            migration_paths: Vec::new(),
            cleanup_tables: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_migrations(mut self, paths: Vec<String>) -> Self {
        self.migration_paths = paths;
        self
    }

    #[must_use]
    pub fn with_cleanup_tables(mut self, tables: Vec<String>) -> Self {
        self.cleanup_tables = tables;
        self
    }
}

/// A running `PostgreSQL` test container with a connection pool.
pub struct PostgresContainer {
    pub config: PostgresConfig,
    #[allow(dead_code)]
    container: ContainerAsync<Postgres>,
    pool: PgPool,
}

impl PostgresContainer {
    /// Start a new `PostgreSQL` container and create a connection pool.
    pub async fn start(config: PostgresConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let module = Postgres::default()
            .with_db_name(&config.database)
            .with_user(&config.username)
            .with_password(&config.password);

        let container = module.with_tag(&config.image_tag).start().await?;

        let host = container.get_host().await?;
        let port = container.get_host_port_ipv4(5432).await?;

        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.username, config.password, host, port, config.database
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&connection_string)
            .await?;

        let mut pg = Self {
            config,
            container,
            pool,
        };

        if !pg.config.migration_paths.is_empty() {
            pg.run_migrations().await?;
        }

        Ok(pg)
    }

    /// Get a reference to the connection pool.
    pub const fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run SQL migration files from the configured paths.
    pub async fn run_migrations(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let paths = self.config.migration_paths.clone();
        for pattern in &paths {
            let mut files: Vec<_> = glob_sql_files(pattern)?;
            files.sort();
            for file in files {
                let sql = tokio::fs::read_to_string(&file).await?;
                tracing::info!("Running migration: {}", file.display());
                sqlx::query(&sql).execute(&self.pool).await?;
            }
        }
        Ok(())
    }

    /// Truncate configured cleanup tables (keeps container running).
    pub async fn cleanup_data(&self) -> Result<(), sqlx::Error> {
        crate::cleanup::truncate_tables(&self.pool, &self.config.cleanup_tables).await
    }
}

fn glob_sql_files(pattern: &str) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();

    let path = Path::new(pattern);
    let dir = path.parent().ok_or("invalid migration path")?;
    let extension = "sql";

    if dir.is_dir() {
        let mut entries: Vec<_> = std::fs::read_dir(dir)?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == extension))
            .map(|e| e.path())
            .collect();
        entries.sort();
        results.extend(entries);
    }

    Ok(results)
}
