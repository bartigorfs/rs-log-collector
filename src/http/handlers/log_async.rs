use async_trait::async_trait;
use crate::http::service;
use crate::models::async_handler::{AsyncDbWriter, AsyncLogListener};
use crate::models::log_evt::LogEvent;
use crate::UNCOMMITTED_LOG;

#[async_trait]
impl AsyncLogListener for AsyncDbWriter {
    async fn call(&self, value: &LogEvent) {
        println!("Listener received: {:?}", value);

        let pool = self.pool.clone();
        let _result = service::sqlx::insert_log(pool, value.clone().timestamp, value.clone().entity, value.clone().data).await;

        match _result {
            Err(e) => {
                println!("{}", e.to_string());
                UNCOMMITTED_LOG.lock().await.push(value.clone());
            }
            Ok(()) => (),
        };
    }
}