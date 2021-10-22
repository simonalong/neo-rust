
use crate::Neo;
use crate::constants::*;
use std::thread::LocalKey;

pub struct SqlBuilder;

impl SqlBuilder {

    // pub fn build_sql(tableName: &str, valueMap: &NeoMap) -> String {
    //     "insert into " + tableName + " (" + buildInsertTable(valueMap.keySet()) + ") values (" + buildInsertValues(valueMap) + ")";
    // }

    // `name`, `group`
    // pub fn build_keys(keys: Vec<String>) -> String {
    //     return keys.stream().map(SqlBuilder::toDbField).collect(Collectors.joining(", "));
    // }

    pub fn build_values() {

    }

    pub fn to_db_field(column: String) -> String {
        let db_type = Neo::db_type.with(|e|e.clone().take());
        println!("===== {}", db_type);
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
