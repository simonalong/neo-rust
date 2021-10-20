use neo_rust::NeoMap;
use std::collections::HashMap;

#[test]
pub fn test1() {
    let neo_map = NeoMap::new();
    // neo_map.put("key", "v");
    neo_map.put_str("key", "v");
    // println!("{}", neo_map.get_str("key"));
    println!("{}", neo_map.get_string("key").unwrap());
}

#[test]
fn test_of() {
    // let neo_map = NeoMap::new().put("k1", "v1").put("k2", "v2");
    // println!("{:?}", neo_map);
}


