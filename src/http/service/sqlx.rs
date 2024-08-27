use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Pool, Sqlite};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn insert_log(
    pool: Arc<Mutex<Pool<Sqlite>>>,
    timestamp: String,
    service_name: String,
    data: String,
) -> Result<(), Error> {
    let pool_guard = pool.lock().await;
    let pool_ref = &*pool_guard;

    sqlx::query("INSERT INTO log (timestamp, entity, log) VALUES ($1, $2, $3)")
        .bind(timestamp)
        .bind(service_name)
        .bind(data)
        .execute(pool_ref)
        .await?;

    Ok(())
}

pub async fn get_log(
    pool: Arc<Mutex<Pool<Sqlite>>>,
    date_from: String,
    date_to: String,
    service_name: String,
) -> Result<(Vec<SqliteRow>), Error> {
    let pool_guard = pool.lock().await;
    let pool_ref = &*pool_guard;

    let result: Vec<SqliteRow> = sqlx::query("SELECT * FROM log WHERE entity = $1 AND timestamp BETWEEN $2 AND $3")
        .bind(service_name)
        .bind(date_from)
        .bind(date_to)
        .fetch_all(pool_ref)
        .await?;

    Ok(result)
}