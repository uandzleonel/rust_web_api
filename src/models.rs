
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable)]
pub struct Rustacean {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
  pub name: String,
  pub email: String,
}

#[derive(Queryable)]
pub struct Create {
  pub id: i32,
  pub rustacean_id: i32,
  pub code: String,
  pub name: String,
  pub version: String,
  pub description: Option<String>,
  pub created_at: NaiveDateTime,
}

#[derive(Queryable)]
#[diesel(table_name=crates)]
pub struct NewCreate {
  pub rustacean_id: i32,
  pub code: String,
  pub name: String,
  pub version: Option<String>,
  pub description: Option<String>,
}
