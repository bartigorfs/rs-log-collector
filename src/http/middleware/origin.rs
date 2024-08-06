use std::collections::HashSet;
use std::sync::Arc;

use bytes::Bytes;
use futures::future::{BoxFuture, FutureExt};
use http_body_util::combinators::BoxBody;
use hyper::{body::Incoming, Request, Response, StatusCode};
use hyper::Error;
use hyper::service::Service;

use crate::utils::hyper_util::empty;

#[derive(Debug, Clone)]
pub struct OriginValidation<S> {
    inner: S,
    client_ip: String,
    trusted_origins: Arc<HashSet<String>>,
}

impl<S> OriginValidation<S> {
    pub fn new(inner: S, client_ip: String, trusted_origins: Arc<HashSet<String>>) -> Self {
        OriginValidation {
            inner,
            client_ip,
            trusted_origins,
        }
    }
}

type Req = Request<Incoming>;

impl<S> Service<Req> for OriginValidation<S>
where
    S: Service<Req, Response = Response<BoxBody<Bytes, Error>>, Error = Error> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: Req) -> Self::Future {
        let client_ip: String = self.client_ip.split(':').next().unwrap_or("").to_string();

        if self.trusted_origins.contains(&client_ip) {
            let fut = self.inner.call(req);
            fut.boxed()
        } else {
            let mut resp = Response::new(empty());
            *resp.status_mut() = StatusCode::UNAUTHORIZED;
            futures::future::ok(resp).boxed()
        }
    }
}