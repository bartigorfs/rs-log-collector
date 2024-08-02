use ntex::web;
use crate::AppState;

#[web::post("/log")]
async fn log(data: web::types::State<AppState>) -> impl web::Responder {
    println!("{}", data.sqlite.is_closed());
    web::HttpResponse::Ok().body("Post LOG!")
}