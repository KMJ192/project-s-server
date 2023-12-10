use std::time::Duration;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

pub mod lab;
use lab::test_server::test_server;

pub mod index;
use index::index_html;

pub mod connector;
pub mod schema;

pub mod auth;
pub mod user;

use auth::auth::{sign_in, sign_out};
use user::user::{create_user, delete_user, get_user, update_user};

#[get("/")]
async fn root() -> impl Responder {
  HttpResponse::Ok().body(index_html())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let connection = connector::main_db::main_db_connection();

  HttpServer::new(|| {
    App::new().service(root).service(
      web::scope("/api")
        .service(test_server)
        .service(web::scope("/auth").service(sign_in).service(sign_out))
        .service(
          web::scope("/user")
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user),
        ),
    )
  })
  .bind(("127.0.0.1", 8080))?
  .workers(1)
  .keep_alive(Duration::from_secs(60))
  .run()
  .await
}
