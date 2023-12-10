use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SignInInfo {
  email: String,
  password: String,
}

#[post("/sign_in")]
pub async fn sign_in(data: web::Json<SignInInfo>) -> impl Responder {
  let SignInInfo { email, password } = data.0;

  HttpResponse::Ok().json(SignInInfo { email, password })
}

#[post("/sign_out")]
pub async fn sign_out() -> impl Responder {
  HttpResponse::Ok().body("sign_out")
}
