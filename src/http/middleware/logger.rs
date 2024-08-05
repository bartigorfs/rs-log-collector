use bytes::Bytes;
use chrono::{DateTime, Utc};
use futures::future::BoxFuture;
use http_body_util::combinators::BoxBody;
use hyper::{Request, body::Incoming, service::Service, Response};

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
    S::Error: std::fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let now: DateTime<Utc> = Utc::now();

        println!("{} Request: {} {} {:?}", now.to_rfc3339(), req.method(), req.uri(), req.version());

        let fut = self.inner.call(req);

        Box::pin(async move {
            let result = fut.await;
            match &result {
                Ok(response) => {
                    println!("{} Response: {:?}", now.to_rfc3339(), response.status());
                }
                Err(err) => {
                    println!("{} Error: {:?}", now.to_rfc3339(), err);
                }
            }
            result
        })
    }
}