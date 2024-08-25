use crate::utils::hyper_util::{full, send_empty_ok, send_json_error_response};
use crate::utils::lz4_util::compress_database;
use crate::{database, ROTATE_ACTIVE};
use base64::engine::general_purpose;
use base64::Engine;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Response, StatusCode};
use sqlx::{Error, Pool, Sqlite};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_log_rotate(
    pool: Arc<Mutex<Pool<Sqlite>>>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let rotate_active = ROTATE_ACTIVE.clone();
    let mut is_active = rotate_active.lock().await;

    if *is_active {
        return Ok(send_json_error_response("Only one active rotate!", StatusCode::INTERNAL_SERVER_ERROR)?);
    }

    *is_active = true;

    let close_pool_result: Result<(), Error> = database::close_pool(&pool).await;

    match close_pool_result {
        Ok(_) => {}
        Err(e) => {
            *is_active = false;
            let error_message: String = format!("Cannot close connection pool: {}", e);
            send_json_error_response(&error_message, StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    let result: Vec<u8> = match compress_database().await {
        Ok(data) => data,
        Err(e) => {
            *is_active = false;
            let error_message: String = format!("Cannot compress database: {}", e);
            return send_json_error_response(&error_message, StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if let Err(e) = database::reinitialize_pool(&pool).await {
        *is_active = false;
        let error_message: String = format!("Cannot reinitialize pool: {}", e);
        return send_json_error_response(&error_message, StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mut resp = Response::new(full(general_purpose::STANDARD.encode(result)));
    *resp.status_mut() = StatusCode::OK;
    *is_active = false;
    Ok(resp)
}
