use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{ AsyncPgConnection, RunQueryDsl };

use crate::schema::*;
use crate::models::*;

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

pub struct CrateRepository;

impl CrateRepository {
  pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
    crates::table.find(id).get_result(conn).await
  }

  pub async fn find_many(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
    crates::table.limit(limit).get_results(conn).await
  }

  pub async fn create(conn: &mut AsyncPgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
    diesel::insert_into(crates::table)
      .values(new_crate)
      .get_result(conn)
      .await
  }

  pub async fn update(conn: &mut AsyncPgConnection, a_crate: Crate) -> QueryResult<Crate> {
    diesel::update(crates::table.find(a_crate.id))
      .set((
        crates::rustacean_id.eq(a_crate.rustacean_id),
        crates::code.eq(a_crate.code),
        crates::name.eq(a_crate.name),
        crates::version.eq(a_crate.version),
        crates::description.eq(a_crate.description),
      ))
      .get_result(conn)
      .await
  }
  
  pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(crates::table.find(id)).execute(conn).await
  }
}
