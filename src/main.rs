// mod database;
// mod zstd_worker;
//
// use crate::database::Database;
// use hyper::body::HttpBody;
// use hyper::service::{make_service_fn, service_fn};
// use hyper::{Body, Request, Response, Server};
// use std::convert::Infallible;
// use std::net::SocketAddr;
// use std::sync::Arc;
// use tokio::sync::Mutex;
// use hyper::server::conn::AddrIncoming; // Add this import
//
//
// async fn handle_request(
//     db: Arc<Mutex<Database>>,
//     req: Request<Body>,
// ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
//     let body = hyper::body::to_bytes(req.into_body()).await?;
//     let db_clone = db.clone();
//     let db_locked = db_clone.lock().await;
//     db_locked.save_data(&body)?;
//     Ok(Response::new(Body::from("Data saved successfully")))
// }
//
// #[tokio::main]
// async fn main() {
//     let db = Database::new("db.json");
//     let db = Arc::new(Mutex::new(db));
//
//     let make_svc = make_service_fn(|_conn| {
//         let db = db.clone();
//         async { Ok::<_, Infallible>(service_fn(move |req| handle_request(db.clone(), req))) }
//     });
//
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//
//     let server_builder = Server::builder(AddrIncoming::bind(&addr)?);
//
//     let server = server_builder.serve(make_svc);
//
//     if let Err(e) = server.await {
//         eprintln!("server error: {}", e);
//     }
//
//     println!("Server running on http://{}", addr);
//     server.await?;
//     Ok(())
// }

mod utils;
mod handlers;

use ntex::web;
use serde::{Deserialize, Serialize};
use sqlx::{Connection, Pool, Sqlite, SqliteConnection, SqlitePool};
use sqlx::sqlite::SqliteConnectOptions;
use crate::utils::zstd_util;

#[web::get("/")]
async fn hello() -> impl web::Responder {

    web::HttpResponse::Ok().body("Hello world!")
}

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hey there!")
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let options = SqliteConnectOptions::new()
        .filename("logDB.db")
        .create_if_missing(true);

    let conn: Pool<Sqlite> = SqlitePool::connect_with(options).await.expect("Database connection is failed");

    web::HttpServer::new(|| {
        web::App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}