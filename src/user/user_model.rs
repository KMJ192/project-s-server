use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[path = "../schema.rs"]
mod schema;

pub enum UserRole {
  Admin,
  User,
}

#[allow(dead_code)]
#[diesel(table_name = users)]
pub struct UserModel {
  id: u16,
  email: String,
  nickname: String,
  password: String,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  role: UserRole,
}
