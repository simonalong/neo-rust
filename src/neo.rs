use sqlx::{Database, Pool, mysql::{MySqlPoolOptions, MySqlQueryResult, MySqlRow}, pool::PoolOptions, postgres::PgPoolOptions, sqlite::SqlitePoolOptions, MySql, Row, Column, TypeInfo};
use std::collections::HashMap;
// use thread_local::ThreadLocal;
use crate::{NeoMap, SqlBuilder, Put};
use serde_json::Value;
use std::cell::RefCell;
use lazy_static::lazy_static;
use std::any::TypeId;

lazy_static! {
    // mysql类型对应rust的默认类型
    pub static ref MYSQL_TYPE_RUST_MAP: HashMap<String, TypeId> = {
        let mut map = HashMap::new();
        map.insert(String::from("BIGINT"), TypeId::of::<i64>());
        map.insert(String::from("BINARY"), TypeId::of::<Vec<u8>>());
        // rust暂时不支持bit
        // map.insert(String::from("BIT", "--");
        map.insert(String::from("BLOB"), TypeId::of::<Vec<u8>>());
        map.insert(String::from("BOOLEAN"), TypeId::of::<bool>());
        map.insert(String::from("CHAR"), TypeId::of::<String>());
        map.insert(String::from("DATE"), TypeId::of::<chrono::NaiveDate>());
        map.insert(String::from("DATETIME"), TypeId::of::<chrono::NaiveDateTime>());
        map.insert(String::from("DECIMAL"), TypeId::of::<bigdecimal::BigDecimal>());
        map.insert(String::from("DOUBLE"), TypeId::of::<i64>());
        // map.insert(String::from("ENUM"), "--");
        map.insert(String::from("FLOAT"), TypeId::of::<f32>());
        // map.insert(String::from("GEOMETRY"), "--");
        // map.insert(String::from("GEOMETRYCOLLECTION"), "--");
        map.insert(String::from("INT"), TypeId::of::<i32>());
        map.insert(String::from("JSON"), TypeId::of::<json::JsonValue>());
        map.insert(String::from("LINESTRING"), TypeId::of::<String>());
        // map.insert(String::from("LONGBLOB"), "--");
        map.insert(String::from("LONGTEXT"), TypeId::of::<String>());
        // map.insert(String::from("MEDIUMBLOB"), "--");
        // map.insert(String::from("MEDIUMINT"), "--");
        // map.insert(String::from("MEDIUMTEXT"), "--");
        // map.insert(String::from("MULTILINESTRING"), "--");
        // map.insert(String::from("MULTIPOINT"), "--");
        // map.insert(String::from("POINT"), "--");
        // map.insert(String::from("POLYGON"), "--");
        map.insert(String::from("SMALLINT"), TypeId::of::<i16>());
        map.insert(String::from("TEXT"), TypeId::of::<String>());
        map.insert(String::from("TIME"), TypeId::of::<chrono::NaiveTime>());
        map.insert(String::from("TIMESTAMP"), TypeId::of::<chrono::DateTime<chrono::Utc>>());
        // map.insert(String::from("TINYBLOB"), "--");
        map.insert(String::from("TINYINT"), TypeId::of::<i8>());
        map.insert(String::from("TINYTEXT"), TypeId::of::<String>());
        map.insert(String::from("TINYINT UNSIGNED"), TypeId::of::<u8>());
        map.insert(String::from("SMALLINT UNSIGNED"), TypeId::of::<u16>());
        map.insert(String::from("INT UNSIGNED"), TypeId::of::<u32>());
        map.insert(String::from("BIGINT UNSIGNED"), TypeId::of::<u64>());
        map.insert(String::from("VARCHAR"), TypeId::of::<String>());
        // map.insert(String::from("YEAR"), "--");
        map
    };
    // // rust类型对应mysql的默认类型
    // pub static ref RUST_TYPE_MYSQL_MAP: HashMap<TypeId, &str> = {
    //     let mut map = HashMap::new();
    //     map.insert(18, "hury".to_owned());
    //     map
    // };
}

pub struct Neo {
    connect_pool: Pool<MySql>,
}

impl Neo {

    thread_local! {
        pub static db_type: RefCell<String> = RefCell::new(String::new());
    }

    pub async fn connect(uri: &str) -> Neo {
        if uri == "" {
            panic!("uri cannot empty")
        }

        if uri.to_string().starts_with("mysql") {
            // println!("-----{}", &uri);
            Neo::db_type.with(|e| (*(e.borrow_mut()) = String::from("mysql")));

            return Neo {
                connect_pool: MySqlPoolOptions::new().connect(uri).await.unwrap(),
            };
            // } else if uri.to_string().starts_with("postgres") {
            //     return &Neo{connect_pool: PgPoolOptions::new().connect(uri).await?}
            // } else if uri.to_string().starts_with("sqlite") {
            //     return &Neo{connect_pool: SqlitePoolOptions::new().connect(uri).await?}
        } else {
            panic!("not support database: {}", uri);
        }
    }

    pub fn get_connect_pool(&self) -> &Pool<MySql> {
        &self.connect_pool
    }

    pub async fn insert(&self, table_name: &str, value_map: NeoMap) {
        let (sql, values) = SqlBuilder::generate_insert_pair(table_name, value_map);

        let query = sqlx::query(sql.as_str());
        for v in values {
            query.bind(v);
        }

        let result = query.execute(self.get_connect_pool()).await;
        let it = result.iter();
        for x in it {
            for v in x {
                let neo_map = generate(v);
                println!(" neo map  === {:?}", neo_map);
            }
        }
        // let result = sqlx::query(sql.as_str()).bind("name1").bind("group1").execute(self.get_connect_pool()).await;
        // println!("end 1");
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
}

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
