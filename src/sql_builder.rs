
use crate::Neo;
use crate::constants::*;
use std::thread::LocalKey;
use string_join::Join;

pub struct SqlBuilder;

impl SqlBuilder {

    // pub fn build_sql(tableName: &str, valueMap: &NeoMap) -> String {
    //     "insert into " + tableName + " (" + buildInsertTable(valueMap.keySet()) + ") values (" + buildInsertValues(valueMap) + ")";
    // }

    // `name`, `group`
    pub fn build_keys(keys: Vec<String>) -> String {
        let data = keys.iter().map(|e|SqlBuilder::to_db_field(e)).collect();
        // todo
    }

    pub fn build_values() {

    }

    pub fn to_db_field(column: &String) -> &String {
        let db_type = Neo::db_type.with(|e|e.clone().take());
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
        return &String::from(s1);
    }
}
