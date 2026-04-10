use std::future::Future;
use std::pin::Pin;

use sqlx::PgPool;
use uuid::Uuid;

use crate::entity::User;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Data access contract for user operations.
///
/// Uses boxed futures to be dyn-compatible (`Arc<dyn UserRepository>`).
pub trait UserRepository: Send + Sync {
    fn find_all(&self) -> BoxFuture<'_, Result<Vec<User>, sqlx::Error>>;
    fn find_by_id(&self, id: Uuid) -> BoxFuture<'_, Result<Option<User>, sqlx::Error>>;
    fn find_by_email(&self, email: &str) -> BoxFuture<'_, Result<Option<User>, sqlx::Error>>;
    fn create(&self, name: &str, email: &str) -> BoxFuture<'_, Result<User, sqlx::Error>>;
    fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        email: Option<&str>,
    ) -> BoxFuture<'_, Result<Option<User>, sqlx::Error>>;
    fn delete(&self, id: Uuid) -> BoxFuture<'_, Result<bool, sqlx::Error>>;
}

#[derive(Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PgUserRepository {
    fn find_all(&self) -> BoxFuture<'_, Result<Vec<User>, sqlx::Error>> {
        Box::pin(async move {
            sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
                .fetch_all(&self.pool)
                .await
        })
    }

    fn find_by_id(&self, id: Uuid) -> BoxFuture<'_, Result<Option<User>, sqlx::Error>> {
        Box::pin(async move {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
        })
    }

    fn find_by_email(&self, email: &str) -> BoxFuture<'_, Result<Option<User>, sqlx::Error>> {
        let email = email.to_owned();
        Box::pin(async move {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
                .bind(&email)
                .fetch_optional(&self.pool)
                .await
        })
    }

    fn create(&self, name: &str, email: &str) -> BoxFuture<'_, Result<User, sqlx::Error>> {
        let name = name.to_owned();
        let email = email.to_owned();
        Box::pin(async move {
            sqlx::query_as::<_, User>("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
                .bind(&name)
                .bind(&email)
                .fetch_one(&self.pool)
                .await
        })
    }

    fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        email: Option<&str>,
    ) -> BoxFuture<'_, Result<Option<User>, sqlx::Error>> {
        let name = name.map(str::to_owned);
        let email = email.map(str::to_owned);
        Box::pin(async move {
            sqlx::query_as::<_, User>(
                "UPDATE users SET \
                    name = COALESCE($2, name), \
                    email = COALESCE($3, email), \
                    updated_at = now() \
                 WHERE id = $1 \
                 RETURNING *",
            )
            .bind(id)
            .bind(name.as_deref())
            .bind(email.as_deref())
            .fetch_optional(&self.pool)
            .await
        })
    }

    fn delete(&self, id: Uuid) -> BoxFuture<'_, Result<bool, sqlx::Error>> {
        Box::pin(async move {
            let result = sqlx::query("DELETE FROM users WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await?;
            Ok(result.rows_affected() > 0)
        })
    }
}
