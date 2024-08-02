mod database;
mod handlers;
mod utils;
mod models;

use dotenv::dotenv;
use ntex::web;
use sqlx::{Connection, Pool, Sqlite};
use std::sync::Arc;
use crate::handlers::log::log;
use crate::handlers::rotate::rotate;
use crate::models::app::AppState;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool: Pool<Sqlite> = database::connect_database().await;

    let port: u16 = std::env::var("PORT")
        .expect("PORT must be set.")
        .parse()
        .unwrap();
    let host: String = std::env::var("HOST").expect("HOST must be set.");

    web::HttpServer::new(move || {
        web::App::new()
            .state(AppState {
                sqlite: Arc::new(pool.clone()),
            })
            .service({
                web::resource("/log").route(web::post().to(log))
            })
            .service(rotate)
    })
    .bind((host, port))?
    .run()
    .await
}
