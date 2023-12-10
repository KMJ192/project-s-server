use diesel::prelude::{Queryable, Selectable};

// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub enum UserRole {
  Admin,
  User,
}

#[allow(dead_code)]
#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::users)]
pub struct UsersModel {
  id: u32,
  email: String,
  nickname: String,
  password: String,
  created_at: String,
  updated_at: String,
  role: u8,
}
