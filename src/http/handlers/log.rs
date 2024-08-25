use crate::http::service;
use crate::utils::hyper_util::{full, send_empty_ok, send_json_error_response};
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::body::Body;
use hyper::{HeaderMap, Request, Response, StatusCode};
use sqlx::{Error, Pool, Sqlite};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_post_log(
    pool: Arc<Mutex<Pool<Sqlite>>>,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let max: u64 = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 256 {    // 256kb
        send_json_error_response("Body too big", StatusCode::PAYLOAD_TOO_LARGE)?;
    }

    let headers: HeaderMap = req.headers().clone();

    let service_name: &str = headers
        .get("Service")
        .and_then(|header_value| header_value.to_str().ok())
        .unwrap_or("Unknown Service");

    let whole_body: Bytes = req.collect().await?.to_bytes();

    let body_str: String = match String::from_utf8(Vec::from(whole_body)) {
        Ok(s) => s,
        Err(_) => {
            let mut resp = Response::new(full("Cannot stringify body"));
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(resp);
        }
    };

    let result: Result<(), Error> = service::sqlx::insert_log(pool, service_name, body_str).await;

    let response = match result {
        Err(e) => {
            let error_message: String = e.to_string();
            send_json_error_response(&error_message, StatusCode::INTERNAL_SERVER_ERROR)?
        }
        Ok(()) => send_empty_ok()?,
    };

    Ok(response)
}
