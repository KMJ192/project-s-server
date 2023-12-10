use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// 데이터베이스 연결 및 초기화
pub fn main_db_connection() -> MysqlConnection {
  dotenv().ok();

  let database_url =
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

  MysqlConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}
