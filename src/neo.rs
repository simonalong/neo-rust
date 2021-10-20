// use std::any::Any;
//
// use sqlx::{Database, MySql, Pool, mysql::{MySqlPoolOptions, MySqlQueryResult, MySqlRow}, pool::PoolOptions, postgres::PgPoolOptions, sqlite::SqlitePoolOptions};
//
// use crate::NeoMap;
//
// pub struct Neo {
//     pub connect_pool: Pool<Database>
//     // impl<DB: Database> PoolOptions<DB>
// }
//
// impl Neo {
//     pub fn new(connect_pool: Pool<MySql>) -> Self { Self { connect_pool } }
//
//     pub async fn connect(&self, uri: &str) -> Neo<MySql> {
//         if uri = "" {
//             panic!("uri cannot empty")
//         }
//         if uri.to_string().starstarts_with("mysql") {
//             self.connect_pool = MySqlPoolOptions::new().connect(uri).await?;
//         } else if uri.to_string().starstarts_with("postgres") {
//             self.connect_pool = PgPoolOptions::new().connect(uri).await?;
//         } else if uri.to_string().starstarts_with("sqlite") {
//             self.connect_pool = SqlitePoolOptions::new().connect(uri).await?;
//         } else {
//             panic!("not support database: {}", uri);
//         }
//         self
//     }
//
//     pub async fn connect_option(&self, pool: &Pool<MySql>) -> Neo<MySql> {
//         self.connect_pool = pool;
//         self
//     }
// }
//
// trait Cqrs {
//     fn insert();
//
//     fn delete();
//
//     // pub fn update() {}
//
//     // pub fn one() {}
//
//     // pub fn list() {}
//
//     // pub fn value() {}
//
//     // pub fn values() {}
//
//     // pub fn page() {}
//
//     // pub fn count() {}
//
//     // pub fn exist() {}
//
//     // pub fn batchInsert() {}
//
//     // pub fn batchUpdate() {}
// }
//
// impl Cqrs for Neo {
//     fn insert(dataMap: NeoMap) {
//
//     }
// }
