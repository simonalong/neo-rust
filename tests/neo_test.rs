use sqlx::{Database, Pool, MySql, Row, Column, TypeInfo};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
use neo_rust::{Neo, NeoMap, Put, MYSQL_TYPE_RUST_MAP};
use std::thread;
use std::time::Duration;
use serde_json::Value;
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::convert::TryInto;


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

    let sql = "select `name` from demo1 where id > ? limit 1";
    // let sql = "select d1.`name`, d2.`name` from demo1 as d1 left join demo2 as d2 on d1.`id` = d2.`id`  where d1.`id` > 1 limit 1";

    let result = sqlx::query(sql).bind(2).fetch_all(neo.get_connect_pool()).await;
    // println!("x ====  {:#?}", result);
    let it = result.iter();
    for x in it {
        for v in x {
            let neo_map = generate(v);
            println!(" neo map  === {:?}", neo_map);
        }
    }
}

pub fn bind_with() {

}

pub fn generate(row: &MySqlRow) -> NeoMap {
    let columns = row.columns();

    let value_map = NeoMap::new();
    for column in columns {
        let column_name = column.name();
        let type_id = *(MYSQL_TYPE_RUST_MAP.get(column.type_info().name()).unwrap());
        let mut value:Value = Default::default();
        if type_id == TypeId::of::<String>() {
            value = Value::from(row.get::<String, &str>(column_name))
        } else if type_id == TypeId::of::<&str>() {
            value = Value::from(row.get::<&str, &str>(column_name))
        } else if type_id == TypeId::of::<i8>() {
            value = Value::from(row.get::<i8, &str>(column_name))
        } else if type_id == TypeId::of::<i16>() {
            value = Value::from(row.get::<i16, &str>(column_name))
        } else if type_id == TypeId::of::<i32>() {
            value = Value::from(row.get::<i32, &str>(column_name))
        } else if type_id == TypeId::of::<i64>() {
            value = Value::from(row.get::<i64, &str>(column_name))
        } else if type_id == TypeId::of::<u8>() {
            value = Value::from(row.get::<u8, &str>(column_name))
        } else if type_id == TypeId::of::<u16>() {
            value = Value::from(row.get::<u16, &str>(column_name))
        } else if type_id == TypeId::of::<u32>() {
            value = Value::from(row.get::<u32, &str>(column_name))
        } else if type_id == TypeId::of::<u64>() {
            value = Value::from(row.get::<u64, &str>(column_name))
        } else if type_id == TypeId::of::<f32>() {
            value = Value::from(row.get::<f32, &str>(column_name))
        } else if type_id == TypeId::of::<f64>() {
            value = Value::from(row.get::<f64, &str>(column_name))
        } else if type_id == TypeId::of::<bool>() {
            value = Value::from(row.get::<bool, &str>(column_name))
        } else {
            value = Default::default()
        }
        value_map.put(column_name, value);
    }

    value_map
}

