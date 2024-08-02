use ntex::web;
use crate::AppState;

#[web::get("/rotate")]
async fn rotate(data: web::types::State<AppState>) -> impl web::Responder {
    println!("{}", data.sqlite.is_closed());
    web::HttpResponse::Ok().body("Rotate!")
}