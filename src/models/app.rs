use std::sync::Arc;
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct AppState {
    pub(crate) sqlite: Arc<Pool<Sqlite>>,
}
