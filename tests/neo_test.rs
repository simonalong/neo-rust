use sqlx::{Database, Pool, MySql, Row, Column};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
use neo_rust::{Neo, NeoMap, Put};
use std::thread;
use std::time::Duration;
use serde_json::Value;
use std::any::Any;
use std::ops::Deref;

#[tokio::test]
async fn test() {
    let neo = Neo::connect("mysql://neo_test:neo@Test123@localhost:3306/demo1").await;

    // 查询：多行
    // let sql = "select `name`, `group` from demo1";
    // let result = sqlx::query(sql).bind("name2").fetch_all(neo.get_connect_pool()).await?;
    // [MySqlRow { row: Row { storage: b"\x08\x05name2", values: [Some(2..7), None] }, format: Binary, columns: [MySqlColumn { ordinal: 0, name: name, type_info: MySqlTypeInfo { type: VarString, flags: NOT_NULL, char_set: 224, max_size: Some(48) }, flags: Some(NOT_NULL) }, MySqlColumn { ordinal: 1, name: group, type_info: MySqlTypeInfo { type: VarString, flags: (empty), char_set: 224, max_size: Some(256) }, flags: Some((empty)) }], column_names: {name: 0, group: 1} }, MySqlRow { row: Row { storage: b"\x08\x05name3", values: [Some(2..7), None] }, format: Binary, columns: [MySqlColumn { ordinal: 0, name: name, type_info: MySqlTypeInfo { type: VarString, flags: NOT_NULL, char_set: 224, max_size: Some(48) }, flags: Some(NOT_NULL) }, MySqlColumn { ordinal: 1, name: group, type_info: MySqlTypeInfo { type: VarString, flags: (empty), char_set: 224, max_size: Some(256) }, flags: Some((empty)) }], column_names: {name: 0, group: 1} }, MySqlRow { row: Row { storage: b"\x08\x05name4", values: [Some(2..7), None] }, format: Binary, columns: [MySqlColumn { ordinal: 0, name: name, type_info: MySqlTypeInfo { type: VarString, flags: NOT_NULL, char_set: 224, max_size: Some(48) }, flags: Some(NOT_NULL) }, MySqlColumn { ordinal: 1, name: group, type_info: MySqlTypeInfo { type: VarString, flags: (empty), char_set: 224, max_size: Some(256) }, flags: Some((empty)) }], column_names: {name: 0, group: 1} }, MySqlRow { row: Row { storage: b"\x08\x072_demo1", values: [Some(2..9), None] }, format: Binary, columns: [MySqlColumn { ordinal: 0, name: name, type_info: MySqlTypeInfo { type: VarString, flags: NOT_NULL, char_set: 224, max_size: Some(48) }, flags: Some(NOT_NULL) }, MySqlColumn { ordinal: 1, name: group, type_info: MySqlTypeInfo { type: VarString, flags: (empty), char_set: 224, max_size: Some(256) }, flags: Some((empty)) }], column_names: {name: 0, group: 1} }, MySqlRow { row: Row { storage: b"\x08\x0512312", values: [Some(2..7), None] }, format: Binary, columns: [MySqlColumn { ordinal: 0, name: name, type_info: MySqlTypeInfo { type: VarString, flags: NOT_NULL, char_set: 224, max_size: Some(48) }, flags: Some(NOT_NULL) }, MySqlColumn { ordinal: 1, name: group, type_info: MySqlTypeInfo { type: VarString, flags: (empty), char_set: 224, max_size: Some(256) }, flags: Some((empty)) }], column_names: {name: 0, group: 1} }, MySqlRow { row: Row { storage: b"\x08\x0512312", values: [Some(2..7), None] }, format: Binary, columns: [MySqlColumn { ordinal: 0, name: name, type_info: MySqlTypeInfo { type: VarString, flags: NOT_NULL, char_set: 224, max_size: Some(48) }, flags: Some(NOT_NULL) }, MySqlColumn { ordinal: 1, name: group, type_info: MySqlTypeInfo { type: VarString, flags: (empty), char_set: 224, max_size: Some(256) }, flags: Some((empty)) }], column_names: {name: 0, group: 1} }, MySqlRow { row: Row { storage: b"\x08\x0512312", values: [Some(2..7), None] }, format: Binary, columns: [MySqlColumn { ordinal: 0, name: name, type_info: MySqlTypeInfo { type: VarString, flags: NOT_NULL, char_set: 224, max_size: Some(48) }, flags: Some(NOT_NULL) }, MySqlColumn { ordinal: 1, name: group, type_info: MySqlTypeInfo { type: VarString, flags: (empty), char_set: 224, max_size: Some(256) }, flags: Some((empty)) }], column_names: {name: 0, group: 1} }, MySqlRow { row: Row { storage: b"\x08\x0512312", values: [Some(2..7), None] }, format: Binary, columns: [MySqlColumn { ordinal: 0, name: name, type_info: MySqlTypeInfo { type: VarString, flags: NOT_NULL, char_set: 224, max_size: Some(48) }, flags: Some(NOT_NULL) }, MySqlColumn { ordinal: 1, name: group, type_info: MySqlTypeInfo { type: VarString, flags: (empty), char_set: 224, max_size: Some(256) }, flags: Some((empty)) }], column_names: {name: 0, group: 1} }]
    // println!("{:?}", result);

    let sql = "select 1";
    let result = sqlx::query(sql).fetch_all(neo.get_connect_pool()).await;
    println!("{:?}", result);
}

// #[tokio::test]
// async fn test_insert() {
//     let neo = Neo::connect("mysql://neo_test:neo@Test123@localhost:3306/demo1").await;
//
//     let data = NeoMap::new();
//     data.put("name", "n");
//     data.put("group", "g");
//     neo.insert("demo1", &data).await;
// }

#[tokio::test]
async fn test_insert() {
    let neo = Neo::connect("mysql://neo_test:neo@Test123@localhost:3306/demo1").await;

    let sql = "select * from demo1 where id > ? limit 1";

    let result = sqlx::query(sql).bind(2).fetch_all(neo.get_connect_pool()).await;
    println!("x ====  {:#?}", result);
    let it = result.iter();
    for x in it {
        for v in x {
            generate(v);

            // let d: &MySqlRow = v;
            // v.columns().into_iter().for_each(|e|{
            //     println!(" = {}", e.name());
            // });
            // println!("v ====  {:#?}", v);
            // println!("v.id ====  {}", v.get::<i8, &str>("id"));
            // println!("v.id ====  {}", v.get::<i16, &str>("id"));
            // println!("v.id ====  {}", v.get::<i32, &str>("id"));
            // println!("v.id ====  {}", v.get::<i64, &str>("id"));
            //
            // println!("v.name ====  {}", v.get::<&str, &str>("name"));
            // println!("v.group ====  {}", v.get::<String, &str>("group"));
        }
    }
}

pub fn generate(row: &MySqlRow) -> NeoMap {
    let columns:Vec<&str> = row.columns().into_iter().map(|e|e.name()).collect();

    for x in columns {
        println!("{}", x);
    }

    NeoMap::new()
}

