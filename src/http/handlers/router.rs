use crate::http::handlers::log::{handle_get_log, handle_post_log};
use crate::http::handlers::rotate::handle_log_rotate;
use crate::utils::hyper_util::{empty, send_empty_ok};
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Method, Request, Response, StatusCode};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use hyper::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN};
use tokio::sync::Mutex;


pub async fn router(
    pool: Arc<Mutex<Pool<Sqlite>>>,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::OPTIONS, _) => {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header(ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS")
                .header(ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type")
                .body(empty())
                .unwrap())
        },

        (&Method::GET, "/health") => send_empty_ok(),

        (&Method::POST, "/log") => handle_post_log(req).await,

        (&Method::GET, "/rotate") => handle_log_rotate(pool).await,

        (&Method::GET, "/query") => handle_get_log(req, pool).await,

        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
