use std::collections::HashSet;
use std::sync::Arc;

pub struct AppConfig {
    pub trusted_origins: Arc<HashSet<String>>,
    pub host: Vec<u16>,
    pub port: u16,
    pub db_path: String,
}