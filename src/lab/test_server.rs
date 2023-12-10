use actix_web::{get, HttpResponse, Responder};

#[get("/test")]
pub async fn test_server() -> impl Responder {
  HttpResponse::Ok().body("test")
}
