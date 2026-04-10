use sqlx::PgPool;

/// Truncate multiple tables with `RESTART IDENTITY CASCADE`.
pub async fn truncate_tables(pool: &PgPool, tables: &[String]) -> Result<(), sqlx::Error> {
    if tables.is_empty() {
        return Ok(());
    }
    let table_list = tables.join(", ");
    let sql = format!("TRUNCATE TABLE {table_list} RESTART IDENTITY CASCADE");
    sqlx::query(&sql).execute(pool).await?;
    Ok(())
}
