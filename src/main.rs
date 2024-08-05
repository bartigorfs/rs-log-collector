mod database;
mod http;
mod service;
mod utils;

use crate::http::api::run_server;
use crate::utils::graceful::get_graceful_signal;
use dotenv::dotenv;
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::{watch, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool: Pool<Sqlite> = database::init_pool().await.expect("Cannot init pool");
    let pool: Arc<Mutex<Pool<Sqlite>>> = Arc::new(Mutex::new(pool));

    let port: u16 = std::env::var("PORT")
        .expect("PORT must be set.")
        .parse()
        .unwrap();
    // let host: String = std::env::var("HOST").expect("HOST must be set.");
    //
    // let host_array: Vec<u16> = host.split(".").map(|s| s.parse::<u16>().unwrap_or(0)).collect::<Vec<u16>>();

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener: TcpListener = TcpListener::bind(addr).await?;

    let (shutdown_tx, mut shutdown_rx) = watch::channel(());
    let shutdown_signal = get_graceful_signal(shutdown_tx);

    tokio::select! {
        _ = shutdown_signal => {
            println!("Received shutdown signal");
        }
        _ = run_server(listener, pool.clone(), &mut shutdown_rx) => {
            println!("Server exited");
        }
    }

    Ok(())
}
