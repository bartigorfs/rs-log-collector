use crate::get_app_config;
use crate::models::app::AppConfig;
use hyper::{body::Incoming, service::Service, Request};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OriginValidation<S> {
    inner: S,
    client_ip: String,
    trusted_origins: HashSet<String>,
}
impl<S> OriginValidation<S> {
    pub fn new(inner: S, client_ip: String, trusted_origins: Arc<_, _>) -> Self {
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
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    fn call(&self, req: Req) -> Self::Future {
        for origin in self.trusted_origins {
            println!("{}", origin);
        }

        println!("{}", self.client_ip);

        self.inner.call(req)
    }
}
