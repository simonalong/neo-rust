// use serde_json::{Number, Value, Map};
// use std::collections::HashMap;
// use dashmap::mapref::one::{Ref, RefMut};
// use dashmap::mapref::entry::Entry;
// use dashmap::lock::{RwLockWriteGuard, RwLockReadGuard};
// use dashmap::iter::{IterMut, Iter};
// use serde::ser::SerializeMap;
// use serde::Serialize;
// use std::ops::Index;
// use std::ops;
// use dashmap::DashMap;
// use serde_json::json;
// use std::borrow::Cow;

use dashmap::DashMap;
use serde_json::{Value, Number};
use std::convert::TryInto;
use std::ops::Index;

/// 提供neo_map["key"]的能力
impl Index<&str> for NeoMap {
    type Output = str;

    fn index(&self, key: &str) -> &str {
        // return &(self.get(key).unwrap())

        "sdf"

        // let v = self.data_map.get(key);
        // if let Some(re) = v {
        //     re.value()
        // } else {
        //     panic!("not value")
        // }
    }
}

pub struct NeoMap {
    data_map: DashMap<String, Value>
}

pub trait Put<T> {
    fn put(&self, key: &str, value: T) -> &Self;
}

impl NeoMap {
    pub fn new() -> Self {
        NeoMap { data_map: DashMap::new() }
    }

    // pub fn of(&[&T]) -> Self {
    //     NeoMap { data_map: DashMap::new() }
    // }


    //
    pub fn contain_key(&self, key: &str) -> bool {
        self.data_map.contains_key(key)
    }

    pub fn del(&self, key: &str) {
        self.data_map.remove(key);
    }


//

// pub fn get_i64(&self, key: &str) -> Box<i64> {
//     Box::new(self.data_map.get(key).unwrap().as_i64().unwrap().clone())
// }
//
//     fn get_i8(&self, key:&str);
//     fn get_i16(&self, key:&str);
//     fn get_i32(&self, key:&str);
//     fn get_i64(&self, key:&str);
//     fn get_i128(&self, key:&str);
//
//     fn get_u8(&self, key:&str);
//     fn get_u16(&self, key:&str);
//     fn get_u32(&self, key:&str);
//     fn get_u64(&self, key:&str);
//     fn get_u128(&self, key:&str);

//     fn get_f32(&self, key:&str);
//     fn get_f64(&self, key:&str);
//
//     fn get_bool(&self, key:&str);
//
//     fn get_vec(&self, key:&str);
//
//     fn get_neo_map(&self, key:&str);

//     fn get_type(&self, key:&str);
}

impl NeoMap {
    pub fn get_i8(&self, key: &str) -> Option<i8> {
        let re = self.get_i64(key);
        if let Some(r) = re {
            Option::Some(r as i8)
        } else {
            Option::None
        }
    }

    pub fn get_i16(&self, key: &str) -> Option<i16> {
        let re = self.get_i64(key);
        if let Some(r) = re {
            Option::Some(r as i16)
        } else {
            Option::None
        }
    }

    pub fn get_i32(&self, key: &str) -> Option<i32> {
        let re = self.get_i64(key);
        if let Some(r) = re {
            Option::Some(r as i32)
        } else {
            Option::None
        }
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        let value_ref = self.data_map.get(key);

        let value = match value_ref {
            Some(data) => {
                data
            }
            _ => {
                return Option::Some(0);
            }
        };
        value.as_i64()
    }


    pub fn get_u8(&self, key: &str) -> Option<u8> {
        let re = self.get_u64(key);
        if let Some(r) = re {
            Option::Some(r as u8)
        } else {
            Option::None
        }
    }

    pub fn get_u16(&self, key: &str) -> Option<u16> {
        let re = self.get_u64(key);
        if let Some(r) = re {
            Option::Some(r as u16)
        } else {
            Option::None
        }
    }

    pub fn get_u32(&self, key: &str) -> Option<u32> {
        let re = self.get_u64(key);
        if let Some(r) = re {
            Option::Some(r as u32)
        } else {
            Option::None
        }
    }

    pub fn get_u64(&self, key: &str) -> Option<u64> {
        let value_ref = self.data_map.get(key);

        let value = match value_ref {
            Some(data) => {
                data
            }
            _ => {
                return Option::Some(0);
            }
        };
        value.as_u64()
    }

    pub fn get_f32(&self, key: &str) -> Option<f32> {
        let re = self.get_f64(key);
        if let Some(r) = re {
            Option::Some(r as f32)
        } else {
            Option::None
        }
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        let value_ref = self.data_map.get(key);

        let value = match value_ref {
            Some(data) => {
                data
            }
            _ => {
                return Option::Some(0.0);
            }
        };
        value.as_f64()
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        let value_ref = self.data_map.get(key);

        let value = match value_ref {
            Some(data) => {
                data
            }
            _ => {
                return Option::Some(false);
            }
        };
        value.as_bool()
    }

    pub fn get_vec(&self, key: &str) -> Option<Vec<Value>> {
        let value_ref = self.data_map.get(key);

        let value = match value_ref {
            Some(data) => {
                data
            }
            _ => {
                return Option::None;
            }
        };
        value.as_array().cloned()
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        let v = self.data_map.get(key);
        if let Some(re) = v {
            if let Some(e) = re.value().as_str() {
                Option::Some(String::from(e))
            } else {
                Option::None
            }
        } else {
            Option::None
        }
    }

    // pub fn get_str(&self, key: &str) -> Option<&str> {
    //     let value_ref = self.data_map.get(key);
    //
    //     let value = match value_ref {
    //         Some(data) => {
    //             data
    //         }
    //         _ => {
    //             return Option::None;
    //         }
    //     };
    //     value.as_str()
    // }

    pub fn get_value(&self, key: &str) -> Option<Value> {
        let v = self.data_map.get("key");
        if let Some(re) = v {
            Option::Some(re.value().clone())
        } else {
            Option::None
        }
    }
}

impl Put<i8> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: i8) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}

impl Put<i16> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: i16) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}

impl Put<i32> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: i32) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}

impl Put<i64> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: i64) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}


impl Put<&i8> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &i8) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(*value));
        self
    }
}

impl Put<&i16> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &i16) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(*value));
        self
    }
}

impl Put<&i32> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &i32) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(*value));
        self
    }
}

impl Put<&i64> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &i64) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(*value));
        self
    }
}

impl Put<u8> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: u8) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}

impl Put<u16> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: u16) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}

impl Put<u32> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: u32) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}

impl Put<u64> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: u64) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}


impl Put<&u8> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &u8) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(*value));
        self
    }
}

impl Put<&u16> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &u16) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(*value));
        self
    }
}

impl Put<&u32> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &u32) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(*value));
        self
    }
}

impl Put<&u64> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &u64) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(*value));
        self
    }
}


impl Put<f32> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: f32) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}

impl Put<f64> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: f64) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}


impl Put<&f32> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &f32) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(*value));
        self
    }
}

impl Put<&f64> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &f64) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(*value));
        self
    }
}

impl Put<bool> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: bool) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}

impl Put<&bool> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &bool) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(*value));
        self
    }
}

impl Put<&str> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &str) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}

impl Put<String> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: String) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}

impl Put<&String> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &String) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value.clone()));
        self
    }
}

impl Put<NeoMap> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: NeoMap) -> &NeoMap {
        let data_map = &value.data_map;
        let mut map: serde_json::Map<String, Value> = serde_json::Map::new();
        for x in data_map {
            map.insert(x.key().clone(), x.value().clone());
        }

        self.data_map.insert(String::from(key), Value::Object(map));
        self
    }
}

impl Put<Value> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: Value) -> &NeoMap {
        self.data_map.insert(String::from(key), value);
        self
    }
}

impl<'a, T: Clone + Into<Value>> Put<Vec<T>> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: Vec<T>) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}

impl<'a, T: Clone + Into<Value>> Put<&Vec<T>> for NeoMap {
    #[inline]
    fn put(&self, key: &str, value: &Vec<T>) -> &NeoMap {
        self.data_map.insert(String::from(key), Value::from(value.clone()));
        self
    }
}
