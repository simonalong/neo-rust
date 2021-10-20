// use serde_json::Value;
// use dashmap::DashMap;
// use std::collections::HashMap;
//
// struct MyClass<'a> {
//     data_map: HashMap<String, &'a Value>
// }
//
// #[test]
// pub fn test1() {
//     let mut m = MyClass::new();
//     m.put("k", "v");
//
//     println!("{}", m.get("k"))
// }
//
// impl<'a> MyClass<'a> {
//     pub fn new() -> Self {
//         MyClass{ data_map: HashMap::new() }
//     }
//
//     pub fn put(&mut self, key: &'a str, value: &'a str) {
//         let data = &'a(Value::from(value)).clone();
//         self.data_map.insert(String::from(key), );
//     }
//
//     pub fn get(&self, key: &str) -> &str {
//         self.data_map.get(key).unwrap().as_str().unwrap()
//
//
//         // s.as_str().clone()
//
//     }
// }
