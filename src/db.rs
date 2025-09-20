use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn get_pg_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment variables");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn check_db_health(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .map(|_| ())
}

pub async fn is_db_healthy(pool: &PgPool) -> bool {
    check_db_health(pool).await.is_ok()
}
