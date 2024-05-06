mod schema;
mod models;
mod repositories;

use repositories::RustaceanRepository;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Value};
use rocket_db_pools::{Connection, Database};
use diesel_async::AsyncPgConnection;

#[derive(Database)]
#[database["rust_web_api_db"]]
struct DBConnection(rocket_db_pools::diesel::PgPool);

#[rocket::get("/rustaceans")]
async fn get_rustaceans(mut db: Connection<DBConnection>) -> Result<Value, Custom<Value>> {
  RustaceanRepository::find_many(&(mut db) as AsyncPgConnection, 100).await
    .map(|rustaceans| json!(rustaceans))
    .map_err(|_| Custom(Status::InternalServerError, json!["Error"]))
}

#[rocket::main]
async fn main() {
  let _ = rocket::build()
    .mount("/", rocket::routes![get_rustaceans])
    .attach(DBConnection::init())
    .launch()
    .await;
}
