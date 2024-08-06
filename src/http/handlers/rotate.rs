use crate::database;
use crate::utils::hyper_util::{full, send_json_error_response};
use crate::utils::lz4_util::compress_database;
use base64::engine::general_purpose;
use base64::Engine;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Response, StatusCode};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_log_rotate(
    pool: Arc<Mutex<Pool<Sqlite>>>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let close_pool_result = database::close_pool(&pool).await;

    match close_pool_result {
        Ok(_) => {}
        Err(e) => {
            let error_message: String = format!("Cannot close connection pool: {}", e);
            send_json_error_response(&error_message, StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    let result: Vec<u8> = match compress_database().await {
        Ok(data) => data,
        Err(e) => {
            let error_message: String = format!("Cannot compress database: {}", e);
            return send_json_error_response(&error_message, StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if let Err(e) = database::reinitialize_pool(&pool).await {
        let error_message: String = format!("Cannot reinitialize pool: {}", e);
        return send_json_error_response(&error_message, StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mut resp = Response::new(full(general_purpose::STANDARD.encode(result)));
    *resp.status_mut() = StatusCode::OK;
    Ok(resp)
}
