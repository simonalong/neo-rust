// use sqlx::{Database, Pool, mysql::{MySqlPoolOptions, MySqlQueryResult, MySqlRow}, pool::PoolOptions, postgres::PgPoolOptions, sqlite::SqlitePoolOptions};
//
// use crate::NeoMap;
//
// pub struct Neo<T>
//     where T: Database
// {
//     pub connect_pool: Pool<T>
// }
//
// impl<T> Neo<T>
//     where T: sqlx::Database
// {
//     pub async fn connect(uri: &str) -> &Neo<T> {
//         if uri == "" {
//             panic!("uri cannot empty")
//         }
//
//         if uri.to_string().starts_with("mysql") {
//             let p = MySqlPoolOptions::new().connect(uri).await?;
//             return &Neo{connect_pool: p}
//         } else if uri.to_string().starts_with("postgres") {
//             return &Neo{connect_pool: PgPoolOptions::new().connect(uri).await?}
//         } else if uri.to_string().starts_with("sqlite") {
//             return &Neo{connect_pool: SqlitePoolOptions::new().connect(uri).await?}
//         } else {
//             panic!("not support database: {}", uri);
//         }
//     }
// }
//
// // trait Cqrs {
// //     fn insert();
// //
// //     fn delete();
// //
// //     // pub fn update() {}
// //
// //     // pub fn one() {}
// //
// //     // pub fn list() {}
// //
// //     // pub fn value() {}
// //
// //     // pub fn values() {}
// //
// //     // pub fn page() {}
// //
// //     // pub fn count() {}
// //
// //     // pub fn exist() {}
// //
// //     // pub fn batchInsert() {}
// //
// //     // pub fn batchUpdate() {}
// // }
//
// impl<T> Neo<T>
//     where T: sqlx::Database
// {
//     fn insert(dataMap: &NeoMap) {
//
//     }
// }
