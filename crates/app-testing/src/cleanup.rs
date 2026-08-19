use sqlx::{AssertSqlSafe, PgPool};

/// Truncate multiple tables with `RESTART IDENTITY CASCADE`.
pub async fn truncate_tables(pool: &PgPool, tables: &[String]) -> Result<(), sqlx::Error> {
    if tables.is_empty() {
        return Ok(());
    }
    let table_list = tables.join(", ");
    let sql = format!("TRUNCATE TABLE {table_list} RESTART IDENTITY CASCADE");
    // Audited (sqlx 0.9 `SqlSafeStr`): `tables` is supplied by the test author via
    // `PostgresConfig::with_cleanup_tables`, never from untrusted input.
    sqlx::query(AssertSqlSafe(sql)).execute(pool).await?;
    Ok(())
}
