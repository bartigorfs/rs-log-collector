use crate::http::handlers::router::router;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use hyper_util::rt::TokioIo;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub async fn run_server(
    listener: TcpListener,
    pool: Arc<Mutex<Pool<Sqlite>>>,
    shutdown_rx: &mut tokio::sync::watch::Receiver<()>,
) {
    loop {
        tokio::select! {
            Ok((stream, _)) = listener.accept() => {
                let io = TokioIo::new(stream);
                let pool = Arc::clone(&pool);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                       .serve_connection(
                            io,
                            service_fn(move |req: Request<hyper::body::Incoming>| {
                                let pool = Arc::clone(&pool);
                                async move {
                                    router(pool, req).await
                                }
                            }),
                        )
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
            }
            _ = shutdown_rx.changed() => {
                println!("Shutdown signal received, stopping server.");
                break;
            }
        }
    }
}
