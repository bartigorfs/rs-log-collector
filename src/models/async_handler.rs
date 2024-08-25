use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use tokio::sync::Mutex;
use crate::models::log_evt::LogEvent;

#[async_trait]
pub trait AsyncListener {
    async fn call(&self, value: &LogEvent);
}

pub struct AsyncDbWriter {
    pub(crate) pool: Arc<Mutex<Pool<Sqlite>>>,
}