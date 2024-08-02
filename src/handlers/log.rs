use ntex::util::BytesMut;
use ntex::web;
use futures::StreamExt;
use ntex::http::Response;
use crate::AppState;
const MAX_SIZE: usize = 262_144; // 256k

pub async fn log(data: web::types::State<AppState>, mut payload: web::types::Payload) -> Result<web::HttpResponse, web::Error> {

    let mut body = BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(web::error::ErrorBadRequest("overflow").into());
        }
        body.extend_from_slice(&chunk);
    }

    let body_str: String = match String::from_utf8(body.to_vec()) {
        Ok(s) => s,
        Err(_) => return Err(web::error::ErrorBadRequest("invalid utf-8").into()),
    };

    let result = sqlx::query(
        "INSERT INTO log (entity, log) VALUES ($1, $2)")
        .bind("TEST".to_string())
        .bind(body_str)
        .execute(&*data.sqlite).await;

    Ok(Response::from(web::HttpResponse::Ok()))
}