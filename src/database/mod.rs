use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::str::FromStr;

pub async fn connect_database() -> Pool<Sqlite> {
    let options: SqliteConnectOptions = SqliteConnectOptions::from_str("sqlite://logDB.db")
        .expect("Failed to create database options")
        .create_if_missing(true);

    let pool: Pool<Sqlite> = SqlitePool::connect_with(options)
        .await
        .unwrap_or_else(|e| panic!("Database connection failed: {:?}", e));

    sqlx::query("CREATE TABLE IF NOT EXISTS log (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
        entity TEXT,
        log TEXT
    );")
        .execute(&pool)
        .await
        .unwrap_or_else(|e| panic!("Cannot create log table: {:?}", e));

    pool
}