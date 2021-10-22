use sqlx::{Database, Pool, mysql::{MySqlPoolOptions, MySqlQueryResult, MySqlRow}, pool::PoolOptions, postgres::PgPoolOptions, sqlite::SqlitePoolOptions, MySql};

use crate::NeoMap;
use serde_json::Value;

pub struct Neo {
    connect_pool: Pool<MySql>
}

impl Neo {
    pub async fn connect(uri: &str) -> Neo {
        if uri == "" {
            panic!("uri cannot empty")
        }

        if uri.to_string().starts_with("mysql") {
            return Neo { connect_pool: MySqlPoolOptions::new().connect(uri).await.unwrap() };
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

    pub async fn insert(&self, tableName: &str, dataMap: &NeoMap) {
        println!("start 1");
        let sql = "insert into demo1(`name`, `group`) values (?, ?)";
        let result = sqlx::query(sql).bind("name1").bind("group1").execute(self.get_connect_pool()).await;
        println!("end 1");
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
