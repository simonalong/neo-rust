use crate::{Neo, NeoMap};
use crate::constants::*;
use std::thread::LocalKey;
use serde_json::Value;

pub struct SqlBuilder;

impl SqlBuilder {

    pub fn generate_insert_pair(table_name: &str, value_map: NeoMap) -> (String, Vec<Value>) {
        (SqlBuilder::build_sql_of_insert(table_name, value_map), value_map.values())
    }

    // 拼接的sql，比如：insert into table1 (`age`, `name`) values (?, ?)
    pub fn build_sql_of_insert(table_name: &str, value_map: NeoMap) -> String {
        let mut sql = "insert into ".to_string();
        sql += table_name;
        sql += " (";
        sql += &SqlBuilder::build_keys(value_map.keys()).as_str();
        sql += ") values (";
        sql += &SqlBuilder::build_values(value_map.keys()).as_str();
        sql += ")";
        sql
    }

    // `name`, `group`
    pub fn build_keys(keys: Vec<String>) -> String {
        let arr: Vec<String> = keys.into_iter().map(|e| SqlBuilder::to_db_field(e)).collect();
        return arr.join(", ");
    }

    pub fn build_values(keys: Vec<String>) -> String {
        let arr: Vec<&str> = keys.into_iter().map(|e| "?").collect();
        return arr.join(", ");
    }

    pub fn to_db_field(column: String) -> String {
        let db_type = Neo::db_type.with(|e| e.clone().take());
        if db_type == POSTGRES {
            return column;
        }

        if column.starts_with("`") || column.ends_with("`") {
            return column;
        }

        let mut s1 = "".to_string();
        s1 += DOM;
        s1 += &column.as_str();
        s1 += DOM;
        return String::from(s1);
    }
}
