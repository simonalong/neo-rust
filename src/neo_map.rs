use serde_json::{Number, Value, Map};
use std::collections::HashMap;
use dashmap::mapref::one::{Ref, RefMut};
use dashmap::mapref::entry::Entry;
use dashmap::lock::{RwLockWriteGuard, RwLockReadGuard};
use dashmap::iter::{IterMut, Iter};
use serde::ser::SerializeMap;
use serde::Serialize;
use std::ops::Index;
use std::ops;
use dashmap::DashMap;
use serde_json::json;
use std::borrow::Cow;

/// 提供neo_map["key"]的能力
// impl Index<&str> for NeoMap {
//     type Output = Value;
//
//     fn index(&self, index: &str) -> &Value {
//         &self.data_map.get(index)
//     }
// }

pub trait Put<T> {
    /// Performs the conversion.
    #[must_use]
    fn put_str(&self, key: &str, value: T) -> &Self;
}

pub struct NeoMap {
    data_map: DashMap<String, Value>
}

impl NeoMap {
    pub fn new() -> Self {
        NeoMap { data_map: DashMap::new() }
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

    // pub fn put(&self, key: &str, value: &str) -> &NeoMap {
    //     self.data_map.insert(String::from(key), Value::from(value));
    //     self
    // }

    // pub fn put_i64(&self, key: &str, value: i64) -> &NeoMap {
    //     self.data_map.insert(String::from(key), Value::from(value).clone());
    //     self
    // }

    pub fn contain_key(&self, key: &str) -> bool {
        self.data_map.contains_key(key)
    }

    pub fn del(&self, key: &str) {
        self.data_map.remove(key);
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        let v = self.data_map.get("key");
        if let Some(re) = v {
            Option::Some(re.value().clone())
        } else {
            Option::None
        }
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        let v = self.data_map.get("key");
        if let Some(re) = v {
            Option::Some(re.value().to_string())
        } else {
            Option::None
        }
    }

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
//
//     fn get_bool(&self, key:&str);
//
//     fn get_vec(&self, key:&str);
//
//     fn get_neo_map(&self, key:&str);
}

// impl Put<i8> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: i8) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// impl Put<i16> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: i16) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// // impl Put<i32> for NeoMap {
// //     #[inline]
// //     fn put(&self, key: &str, value: i32) -> &NeoMap {
// //         self.data_map.insert(String::from(key), Value::from(value));
// //         self
// //     }
// // }
//
// impl Put<i64> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: i64) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// // impl Put<i128> for NeoMap {
// //     #[inline]
// //     fn put(&self, key: &str, value: i128) -> &NeoMap {
// //         self.data_map.insert(String::from(key), Value::from(value));
// //         self
// //     }
// // }
//
//
// impl Put<u8> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: u8) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// impl Put<u16> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: u16) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// impl Put<u32> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: u32) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// impl Put<u64> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: u64) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// impl Put<f32> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: f32) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// impl Put<f64> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: f64) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// // impl Put<u128> for NeoMap {
// //     #[inline]
// //     fn put(&self, key: &str, value: u128) -> &NeoMap {
// //         self.data_map.insert(String::from(key), Value::from(value));
// //         self
// //     }
// // }
//
// impl Put<bool> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: bool) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// // impl Put<char> for NeoMap {
// //     #[inline]
// //     fn put(&self, key: &str, value: char) -> &NeoMap {
// //         self.data_map.insert(String::from(key), Value::from(value));
// //         self
// //     }
// // }
//
// impl Put<Number> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: Number) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// impl Put<&str> for NeoMap {
//     #[inline]
//     fn put_str(&self, key: &str, value: &str) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }

impl<'a> Put<&'a str> for NeoMap {

    fn put_str(&self, key: &str, value: &'a str) -> &Self {
        self.data_map.insert(String::from(key), Value::from(value));
        self
    }
}
//
// // impl<'a> Put<Cow<&'a str>> for NeoMap {
// //     #[inline]
// //     fn put(&self, key: &str, value: Cow<'a, &str>) -> &NeoMap {
// //         self.data_map.insert(String::from(key), Value::from(value));
// //         self
// //     }
// // }
//
// impl Put<String> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: String) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// impl Put<NeoMap> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: NeoMap) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// impl Put<Value> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: Value) -> &NeoMap {
//         self.data_map.insert(String::from(key), value);
//         self
//     }
// }
//
// impl<'a, T: Clone+ Into<Value>> Put<Vec<T>> for NeoMap {
//     #[inline]
//     fn put(&self, key: &str, value: Vec<T>) -> &NeoMap {
//         self.data_map.insert(String::from(key), Value::from(value));
//         self
//     }
// }
//
// // impl<T: Clone> Put<&[T]> for NeoMap {
// //     #[inline]
// //     fn put(&self, key: &str, value: &[T]) -> &NeoMap {
// //         self.data_map.insert(String::from(key), Value::from(value));
// //         self
// //     }
// // }
//
// impl From<NeoMap> for Value {
//     fn from(neo_map: NeoMap) -> Self {
//         let data_map = &neo_map.data_map;
//         let mut map: serde_json::Map<String, Value> = serde_json::Map::new();
//         for x in data_map {
//             map.insert(x.key().clone(), x.value().clone());
//         }
//
//         Value::Object(map)
//     }
// }
