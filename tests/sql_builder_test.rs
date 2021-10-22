use neo_rust::{SqlBuilder, Neo, NeoMap, Put};

#[test]
fn test1() {
    let data = SqlBuilder::to_db_field(String::from("name"));
    assert_eq!("`name`", data)
}

#[test]
fn test2() {
    let mut keys = Vec::new();
    keys.push(String::from("name"));
    keys.push(String::from("group"));
    let data = SqlBuilder::build_keys(keys);
    assert_eq!("`name`, `group`", data)
}

#[test]
fn test3() {
    let value_map = NeoMap::new();
    value_map.put("name", "name1");
    value_map.put("group", "group1");

    let sql = SqlBuilder::build_sql("demo1", value_map);
    assert_eq!("insert into demo1 (`name`, `group`) values (?, ?)", sql)
}
