
struct InsertBuilder;

impl InsertBuilder {

    // pub fn build_sql(tableName: &str, valueMap: &NeoMap) -> String {
    //     "insert into " + tableName + " (" + buildInsertTable(valueMap.keySet()) + ") values (" + buildInsertValues(valueMap) + ")";
    // }

    // `name`, `group`
    pub fn build_keys(keys: Vec<String>) -> String {
        return keys.stream().map(SqlBuilder::toDbField).collect(Collectors.joining(", "));
    }

    pub fn build_values() {

    }
}
