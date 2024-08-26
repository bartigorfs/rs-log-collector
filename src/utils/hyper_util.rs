use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{header, Error, Response, StatusCode};
use hyper::header::HeaderValue;
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

    let body_str: String = body.to_string();

    let body_bytes: Bytes = Bytes::from(body_str);

    let mut resp = Response::new(full(body_bytes));

    *resp.status_mut() = status_code;

    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );

    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, OPTIONS"),
    );
    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Content-Type"),
    );

    Ok(resp)
}

pub fn send_success_with_payload(
    data: String,
    status_code: StatusCode,
) -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    let body_bytes: Bytes = Bytes::from(data);

    let mut resp = Response::new(full(body_bytes));

    *resp.status_mut() = status_code;

    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );

    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, OPTIONS"),
    );
    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Content-Type"),
    );

    Ok(resp)
}

pub fn send_empty_ok() -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    let mut resp = Response::new(empty());
    *resp.status_mut() = StatusCode::OK;

    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, OPTIONS"),
    );
    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Content-Type"),
    );

    Ok(resp)
}