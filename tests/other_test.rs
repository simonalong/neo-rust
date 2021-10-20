use dashmap::{DashMap, Map};
use std::collections::HashMap;
use serde_json::Value;
use serde_json::map::Entry::Vacant;
//
// pub struct Parent<'a>{
//     data: DashMap<&'a str, &'a str>
//     // data: HashMap<String, String>
// }
//
// impl<'a > Parent<'a > {
//     pub fn new() -> Self {
//         // let mut hash = HashMap::new();
//         let hash = DashMap::new();
//         hash.insert("key", "value");
//         Parent { data: hash }
//     }
//
//     pub fn get_value(&self) -> &str {
//         self.data.get("key").unwrap().value()
//     }
// }

#[test]
fn test3() {
    let p = Parent::new();
    println!("{}", p.get_str("key"));
}

pub struct Parent<'a>{
    data: DashMap<&'a str, Value>
    // data: HashMap<String, String>
}

impl<'a> Parent<'a> {
    pub fn new() -> Self {
        // let mut hash = HashMap::new();
        let hash = DashMap::new();
        hash.insert("key", Value::from("value"));
        Parent { data: hash }
    }

    pub fn get_value(&self) -> Value {
        let d = self.data.get("key");
        let m = d.unwrap();
        m.value().clone()
        // m.value().clone().as_str().unwrap()

        // self.data.get("key").unwrap().value().as_str()
    }

    pub fn get_string(&self) -> String {
        let d = self.data.get("key");
        let m = d.unwrap();
        let v = m.value();
        v.to_string()
        // m.value().clone().as_str().unwrap()

        // self.data.get("key").unwrap().value().as_str()
    }
}

pub struct Parent1{
    data: Value
}

impl Parent1 {
    pub fn new() -> Self {
        Parent1 { data: Value::from("sdf") }
    }

    pub fn get_value(&self) -> &str {
        let value = self.data.as_str();

        if let Some(v) = value {
            return v
        } else {
            return ""
        }
    }
}

// pub struct Parent2{
//     data: DashMap<String, String>
// }
//
// impl Parent2 {
//     pub fn new() -> Self {
//         let hash = DashMap::new();
//         hash.insert(String::from("key"), String::from("value"));
//         Parent2 { data: hash }
//     }
//
//     pub fn get_value(&self) -> &str {
//         let value = self.data.get("key").unwrap();
//
//         if let Some(v) = value {
//             return v
//         } else {
//             return ""
//         }
//     }
// }
//
//
// #[test]
// fn test1() {
//     let p = Parent1::new();
//     println!("{}", p.get_value());
// }
//
// #[test]
// fn test2() {
//     let p = Parent2::new();
//     println!("{}", p.get_value());
// }


