use crate::{LOG_EVENT_BUS, UNCOMMITTED_LOG};

pub(crate) async fn process_log_queue() {
    let logs = UNCOMMITTED_LOG.lock().await;

    let length = logs.len();

    if length > 0 {
        for log in logs.iter().cloned() {
            let log_event = log.clone();

            tokio::spawn(async move {
                LOG_EVENT_BUS.push(log_event).await;
            });
        }
    }
}