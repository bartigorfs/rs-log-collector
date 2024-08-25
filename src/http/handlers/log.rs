use crate::models::log_evt::LogEvent;
use crate::utils::hyper_util::{full, send_empty_ok, send_json_error_response};
use crate::LOG_EVENT_BUS;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::body::Body;
use hyper::{HeaderMap, Request, Response, StatusCode};

pub async fn handle_post_log(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let max: u64 = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 256 {    // 256kb
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

    tokio::spawn(async move {
        LOG_EVENT_BUS.push(LogEvent {
            entity: service_name,
            data: body_str,
        }).await;
    });

    send_empty_ok()
}
