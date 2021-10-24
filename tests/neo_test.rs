use sqlx::{Database, Pool, MySql, Row, Column, TypeInfo};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
use neo_rust::{Neo, NeoMap, Put};
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
            // let neo_map = generate("demo1", v);
            // println!(" neo map  === {:?}", neo_map);

            // let d: &MySqlRow = v;
            // v.columns().into_iter().for_each(|e|{
            //     println!(" = {}", e.name());
            // });
            // println!("v ====  {:#?}", v);
            // println!("v.id ====  {:?}", v.type_id());
            // println!("v.id ====  {}", v.column("name").);

            // let m = String::from("asdf");
            let s = v.columns();
            for x in s {
                // println!("v.type_info ====  {:#?}", x.type_info());
                let a = x.type_info();
                println!("v.type_info ====  {}", a.name());
            }
            // println!("v.id ====  {:#?}", v.columns());
            // println!("v.id ====  {:?}", v().type_id());
            // println!("v.id ====  {}", v.get::<i16, &str>("id"));
            // println!("v.id ====  {}", v.get::<i32, &str>("id"));
            // println!("v.id ====  {}", v.get::<i64, &str>("id"));
            //
            // println!("v.name ====  {}", v.get::<&str, &str>("name"));
            // println!("v.group ====  {}", v.get::<String, &str>("group"));
        }
    }
}

pub fn generate(table_name: &str, row: &MySqlRow) -> NeoMap {
    let columns: Vec<&str> = row.columns().into_iter().map(|e| e.name()).collect();

    let value_map = NeoMap::new();
    for x in columns {
        println!("column: {}", x);
        let type_id = get_type_id_from_column(table_name, x);
        println!("type_id: {:?}", type_id);

        let mut value:Value = Default::default();
        if type_id == TypeId::of::<String>() {
            value = Value::from(row.get::<String, &str>(x))
        } else if type_id == TypeId::of::<&str>() {
            value = Value::from(row.get::<&str, &str>(x))
        } else if type_id == TypeId::of::<i8>() {
            value = Value::from(row.get::<i8, &str>(x))
        } else if type_id == TypeId::of::<i16>() {
            value = Value::from(row.get::<i16, &str>(x))
        } else if type_id == TypeId::of::<i32>() {
            value = Value::from(row.get::<i32, &str>(x))
        } else if type_id == TypeId::of::<i64>() {
            value = Value::from(row.get::<i64, &str>(x))
        } else if type_id == TypeId::of::<u8>() {
            value = Value::from(row.get::<u8, &str>(x))
        } else if type_id == TypeId::of::<u16>() {
            value = Value::from(row.get::<u16, &str>(x))
        } else if type_id == TypeId::of::<u32>() {
            value = Value::from(row.get::<u32, &str>(x))
        } else if type_id == TypeId::of::<u64>() {
            value = Value::from(row.get::<u64, &str>(x))
        } else if type_id == TypeId::of::<f32>() {
            value = Value::from(row.get::<f32, &str>(x))
        } else if type_id == TypeId::of::<f64>() {
            value = Value::from(row.get::<f64, &str>(x))
        } else if type_id == TypeId::of::<bool>() {
            value = Value::from(row.get::<bool, &str>(x))
        } else {
            value = Default::default()
        }
        value_map.put(x, value);
    }

    value_map
}

pub fn get_type_id_from_column(table_name: &str, column_name: &str) -> TypeId {
    // todo
    if column_name == "name" || column_name == "group" {
        return TypeId::of::<String>();
    } else {
        return TypeId::of::<i64>();
    }

    // constants::MYSQL_TYPE_RUST


}

