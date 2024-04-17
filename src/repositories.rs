use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{ AsyncPgConnection, RunQueryDsl };

use crate::{models::{NewRustacean, Rustacean}, schema::{crates::table, rustaceans}};

pub struct RustaceanRepository;

impl RustaceanRepository {
  pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
    rustaceans::table.find(id).get_result(conn).await
  }

  pub async fn find_many(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
    rustaceans::table.limit(limit).get_results(conn).await
  }

  pub async fn create(conn: &mut AsyncPgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
    diesel::insert_into(rustaceans::table)
      .values(new_rustacean)
      .get_result(conn)
      .await
  }

  pub async fn update(conn: &mut AsyncPgConnection, rustacean: Rustacean) -> QueryResult<Rustacean> {
    diesel::update(rustaceans::table.find(rustacean.id))
      .set((
        rustaceans::name.eq(rustacean.name),
        rustaceans::email.eq(rustacean.email)
      ))
      .get_result(conn)
      .await
  }

  pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(rustaceans::table.find(id)).execute(conn).await
  }
}
