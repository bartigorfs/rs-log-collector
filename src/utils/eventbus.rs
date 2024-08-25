use std::sync::{Arc};
use tokio::sync::Mutex;
use crate::models::async_handler::AsyncListener;
use crate::models::log_evt::LogEvent;

pub type Listener<T> = Box<dyn Fn(&T) + Send + Sync>;

pub struct EventBus {
    listeners: Arc<Mutex<Vec<Arc<dyn AsyncListener + Send + Sync>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn push(&self, value: LogEvent) {
        let listeners = self.listeners.lock().await;
        for listener in listeners.iter() {
            listener.call(&value).await;
        }
    }

    pub async fn subscribe<F>(&self, listener: F)
    where
        F: AsyncListener + 'static + Send + Sync,
    {
        let mut listeners = self.listeners.lock().await;
        listeners.push(Arc::new(listener));    }
}
