use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;

#[put("/update_user")]
pub async fn update_user() -> impl Responder {
  HttpResponse::Ok().body("update_user")
}

#[post("/create_user")]
pub async fn create_user() -> impl Responder {
  HttpResponse::Ok().body("create_user")
}

#[delete("/delete_user")]
pub async fn delete_user() -> impl Responder {
  HttpResponse::Ok().body("delete_user")
}

#[derive(Deserialize)]
struct GetUserParam {
  email: String,
}

#[get("/get_user")]
pub async fn get_user(param: web::Query<GetUserParam>) -> impl Responder {
  println!("{:?}", param.email);
  HttpResponse::Ok().body("user")
}
