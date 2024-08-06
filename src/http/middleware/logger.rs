use bytes::Bytes;
use futures::future::BoxFuture;
use http_body_util::combinators::BoxBody;
use hyper::{body::Incoming, service::Service, Request, Response};
use chrono::{DateTime, Utc};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Logger<S> {
    inner: S,
}

impl<S> Logger<S> {
    pub fn new(inner: S) -> Self {
        Logger { inner }
    }
}

impl<S> Service<Request<Incoming>> for Logger<S>
where
    S: Service<Request<Incoming>, Response = Response<BoxBody<Bytes, hyper::Error>>>,
    S::Future: Send + 'static,
    S::Error: Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let method = req.method().clone();
        let uri = req.uri().clone();
        let version = req.version();
        let now: DateTime<Utc> = Utc::now();

        let fut = self.inner.call(req);

        Box::pin(async move {
            let result = fut.await;
            match &result {
                Ok(response) => {
                    println!(
                        "{} Request: {} {} {:?} Status: {:?}",
                        now.to_rfc3339(),
                        method,
                        uri,
                        version,
                        response.status()
                    );
                }
                Err(_) => {
                    println!(
                        "{} Request failed: {} {} {:?}",
                        now.to_rfc3339(),
                        method,
                        uri,
                        version
                    );
                }
            }
            result
        })
    }
}