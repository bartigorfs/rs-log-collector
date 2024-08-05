use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{header, Error, Response, StatusCode};
use serde_json::{json};

pub fn empty() -> BoxBody<Bytes, Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub fn send_json_error_response(
    msg: &str,
    status_code: StatusCode,
) -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    let body = json!({
        "message": msg
    });

    let body_str = body.to_string();

    let body_bytes = Bytes::from(body_str);

    let mut resp = Response::new(full(body_bytes));

    *resp.status_mut() = status_code;

    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/json").unwrap(),
    );

    Ok(resp)
}

pub fn send_empty_ok() -> Result<Response<BoxBody<Bytes, Error>>, Error>  {
    let mut resp = Response::new(empty());
    *resp.status_mut() = StatusCode::OK;
    Ok(resp)
}