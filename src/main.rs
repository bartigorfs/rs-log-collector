mod database;
mod http;
mod models;
mod utils;

use crate::http::api::run_server;
use crate::models::app::AppConfig;
use crate::utils::graceful::get_graceful_signal;
use dotenv::dotenv;
use lazy_static::lazy_static;
use sqlx::{Pool, Sqlite};
use std::collections::HashSet;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use chrono::Utc;
use tokio::net::TcpListener;
use tokio::sync::{watch, Mutex};
use crate::models::async_handler::AsyncDbWriter;
use crate::models::log_evt::LogEvent;
use crate::utils::eventbus::EventBus;

lazy_static! {
    pub static ref LOG_EVENT_BUS: EventBus = EventBus::new();
    pub static ref UNCOMMITTED_LOG: Mutex<Vec<LogEvent>> = Mutex::new(Vec::new());
    pub static ref ROTATE_ACTIVE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    static ref APP_CONFIG: AppConfig = {
        dotenv().ok();

        let app_port: u16 = env::var("PORT")
            .expect("PORT must be set.")
            .parse()
            .unwrap();

        let host: String = env::var("HOST").expect("HOST must be set.");

        let db_path: String = env::var("DB_PATH").expect("DB_PATH must be set.");

        let trusted_origins_str: String =
            env::var("TRUSTED_ORIGINS").expect("TRUSTED_ORIGINS must be set.");

        let trusted_origins: HashSet<String> = trusted_origins_str
            .split(',')
            .map(|origin| origin.to_string())
            .collect();

        let host_array: Vec<u16> = host
            .split(".")
            .map(|s| s.parse::<u16>().unwrap_or(0))
            .collect::<Vec<u16>>();

        AppConfig {
            trusted_origins: Arc::new(trusted_origins),
            host: host_array,
            port: app_port,
            db_path,
        }
    };
}

pub async fn get_app_config() -> &'static AppConfig {
    &APP_CONFIG
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    let config: &AppConfig = get_app_config().await;

    let pool: Pool<Sqlite> = database::init_pool().await.expect("Cannot init pool");
    let pool: Arc<Mutex<Pool<Sqlite>>> = Arc::new(Mutex::new(pool));

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener: TcpListener = TcpListener::bind(addr).await?;


    let db_writer = AsyncDbWriter {
        pool: pool.clone(),
    };

    LOG_EVENT_BUS.subscribe(db_writer).await;

    let (shutdown_tx, mut shutdown_rx) = watch::channel(());
    let shutdown_signal = get_graceful_signal(shutdown_tx);

    let message = format!("{tz} Server started on {}", config.port, tz = Utc::now());
    println!("{}", message);

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
