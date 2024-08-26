use std::collections::HashSet;
use std::sync::Arc;
use serde::Serialize;

pub struct AppConfig {
    pub trusted_origins: Arc<HashSet<String>>,
    pub host: Vec<u16>,
    pub port: u16,
    pub db_path: String,
}

#[derive(Serialize)]
pub struct LogEntry {
    pub id: i32,
    pub entity: String,
    pub timestamp: String,
    pub log: String,
}