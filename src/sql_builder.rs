
use crate::Neo;
use crate::DbType;
use crate::constants::*;
struct SqlBuilder;

impl SqlBuilder {

    // pub fn build_sql(tableName: &str, valueMap: &NeoMap) -> String {
    //     "insert into " + tableName + " (" + buildInsertTable(valueMap.keySet()) + ") values (" + buildInsertValues(valueMap) + ")";
    // }

    // `name`, `group`
    pub fn build_keys(keys: Vec<String>) -> String {
        return keys.stream().map(SqlBuilder::toDbField).collect(Collectors.joining(", "));
    }

    pub fn build_values() {

    }

    pub fn to_db_field(column: String) -> String {
        let db_type = Neo::db_type.with(|x|x).take();
        if db_type == DbType::Postgres {
            return column;
        }

        if (column.startsWith(DOM) || column.endsWith(DOM)) {
            return column;
        }
        return "`" + column + "`";
    }
}