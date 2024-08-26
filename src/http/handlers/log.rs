use std::collections::HashMap;
use std::sync::Arc;
use crate::models::log_evt::LogEvent;
use crate::utils::hyper_util::{full, send_empty_ok, send_json_error_response, send_success_with_payload};
use crate::{LOG_EVENT_BUS, ROTATE_ACTIVE, UNCOMMITTED_LOG};
use bytes::Bytes;
use chrono::Utc;
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::body::Body;
use hyper::{HeaderMap, Request, Response, StatusCode};
use sqlx::{Pool, Row, Sqlite};
use tokio::sync::Mutex;
use crate::http::service::sqlx::get_log;
use crate::models::app::LogEntry;

pub async fn handle_post_log(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let cl_rotate = ROTATE_ACTIVE.clone();
    let rotate_active = cl_rotate.lock().await;

    let max: u64 = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 256 {
        send_json_error_response("Body too big", StatusCode::PAYLOAD_TOO_LARGE)?;
    }

    let headers: HeaderMap = req.headers().clone();

    let service_name: String = headers
        .get("Service")
        .and_then(|header_value| header_value.to_str().ok())
        .unwrap_or("Unknown Service").to_string();

    let whole_body: Bytes = req.collect().await?.to_bytes();

    let body_str: String = match String::from_utf8(Vec::from(whole_body)) {
        Ok(s) => s,
        Err(_) => {
            let mut resp = Response::new(full("Cannot stringify body"));
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(resp);
        }
    };

    let now = Utc::now().naive_utc();

    let timestamp_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let log_evt: LogEvent = LogEvent {
        timestamp: timestamp_str,
        entity: service_name,
        data: body_str,
    };

    if *rotate_active {
        UNCOMMITTED_LOG.lock().await.push(log_evt.clone());
    } else {
        tokio::spawn(async move {
            LOG_EVENT_BUS.push(log_evt).await;
        });
    }

    send_empty_ok()
}

pub async fn handle_get_log(
    req: Request<hyper::body::Incoming>,
    pool: Arc<Mutex<Pool<Sqlite>>>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut logs: Vec<LogEntry> = Vec::new();

    let mut date_from: String = "".to_string();
    let mut date_to: String = "".to_string();
    let mut service_name: String = "".to_string();

    if let Some(params) = req.extensions().get::<HashMap<String, String>>() {
        if let Some(dateFrom) = params.get("dateFrom") {
            date_from = dateFrom.clone()
        } else {
            return send_json_error_response("Parameter dateFrom is empty", StatusCode::BAD_REQUEST);
        }

        if let Some(dateTo) = params.get("dateTo") {
            date_to = dateTo.clone()
        } else {
            return send_json_error_response("Parameter dateTo is empty", StatusCode::BAD_REQUEST);
        }

        if let Some(serviceName) = params.get("serviceName") {
            service_name = serviceName.clone()
        } else {
            return send_json_error_response("Parameter serviceName is empty", StatusCode::BAD_REQUEST);
        }
    }

    let logs = match get_log(pool, date_from, date_to, service_name).await {
        Ok(rows) => {
            rows.into_iter()
                .map(|row| LogEntry {
                    id: row.get("id"),
                    entity: row.get("entity"),
                    timestamp: row.get("timestamp"),
                    log: row.get("log"),
                })
                .collect::<Vec<LogEntry>>()
        }
        Err(_) => {
            return Ok(send_json_error_response(
                "Error while acquiring DB connection",
                StatusCode::INTERNAL_SERVER_ERROR,
            )?);
        }
    };

    // Сериализуем логи в JSON
    match serde_json::to_string(&logs) {
        Ok(json_string) => Ok(send_success_with_payload(json_string, StatusCode::OK)?),
        Err(_) => Ok(send_json_error_response(
            "Error serializing logs to JSON",
            StatusCode::INTERNAL_SERVER_ERROR,
        )?),
    }

}