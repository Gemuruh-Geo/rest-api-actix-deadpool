use actix_web::{get, Responder, HttpResponse};

#[get("/health_check")]
async fn do_health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}