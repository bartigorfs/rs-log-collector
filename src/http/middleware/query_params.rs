use std::collections::HashMap;
use std::fmt::Debug;

use bytes::Bytes;
use futures::future::BoxFuture;
use futures::FutureExt;
use http_body_util::combinators::BoxBody;
use hyper::{body::Incoming, Request, Response, service::Service};
use url::Url;

#[derive(Debug, Clone)]
pub struct QueryParams<S> {
    inner: S,
}

impl<S> QueryParams<S> {
    pub fn new(inner: S) -> Self {
        QueryParams { inner }
    }
}

impl<S> Service<Request<Incoming>> for QueryParams<S>
where
    S: Service<Request<Incoming>, Response=Response<BoxBody<Bytes, hyper::Error>>>,
    S::Future: Send + 'static,
    S::Error: Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, mut req: Request<Incoming>) -> Self::Future {
        let base_url: Url = Url::parse("http://localhost").unwrap();
        let binding: Url = base_url.join(req.uri().to_string().as_str()).unwrap();

        let query_params_map: HashMap<String, String> = binding
            .query_pairs()
            .into_owned()
            .collect();

        req.extensions_mut().insert(query_params_map.clone());

        self.inner.call(req).boxed()
    }
}