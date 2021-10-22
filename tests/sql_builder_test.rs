use neo_rust::{SqlBuilder, Neo};

#[test]
fn test1() {
    let data = SqlBuilder::to_db_field(&String::from("name"));
    assert_eq!("`name`", data)
}

#[test]
fn test2() {
    let mut keys = Vec::new();
    keys.push(String::from("name"));
    keys.push(String::from("group"));
    let data = SqlBuilder::build_keys(keys);
    assert_eq!("`name`", data)
}


