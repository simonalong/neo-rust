use std::any::{Any, TypeId};
use serde_json::Value;
use crate::NeoMap;

pub struct ObjectUtil;

impl ObjectUtil {
    // 判断是否是rust基本类型
    pub fn type_is_base<T>(value: T) -> bool
        where T: Any
    {
        let value_any = &value as &dyn Any;
        if let Some(_) = value_any.downcast_ref::<i8>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<i16>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<i32>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<i64>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<isize>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<u8>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<u16>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<u32>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<u64>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<usize>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<f32>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<f64>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<bool>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<&str>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<isize>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<usize>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<char>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<()>() {
            return true;
        } else {
            false
        }
    }

    // 是否是 serde_json::Value可以from直接解析的类型
    pub fn type_is_value_can_from<T>(value: T) -> bool
        where T: Any
    {
        let value_any = &value as &dyn Any;
        if let Some(_) = value_any.downcast_ref::<i8>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<i16>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<i32>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<i64>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<isize>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<u8>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<u16>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<u32>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<u64>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<usize>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<f32>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<f64>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<bool>() {
            return true;
        } else if let Some(_) = value_any.downcast_ref::<&str>() {
            return true;
        } else {
            false
        }
    }

    pub fn type_is_i8<T>(value: T) -> bool where T: Any {
        TypeId::of::<i8>() == value.type_id()
    }

    pub fn type_is_i16<T>(value: T) -> bool where T: Any {
        TypeId::of::<i16>() == value.type_id()
    }

    pub fn type_is_i32<T>(value: T) -> bool where T: Any {
        TypeId::of::<i32>() == value.type_id()
    }

    pub fn type_is_i64<T>(value: T) -> bool where T: Any {
        TypeId::of::<i64>() == value.type_id()
    }

    pub fn type_is_isize<T>(value: T) -> bool where T: Any {
        TypeId::of::<isize>() == value.type_id()
    }

    pub fn type_is_u8<T>(value: T) -> bool where T: Any {
        TypeId::of::<u8>() == value.type_id()
    }

    pub fn type_is_u16<T>(value: T) -> bool where T: Any {
        TypeId::of::<u16>() == value.type_id()
    }

    pub fn type_is_u32<T>(value: T) -> bool where T: Any {
        TypeId::of::<u32>() == value.type_id()
    }

    pub fn type_is_u64<T>(value: T) -> bool where T: Any {
        TypeId::of::<u64>() == value.type_id()
    }

    pub fn type_is_usize<T>(value: T) -> bool where T: Any {
        TypeId::of::<usize>() == value.type_id()
    }

    pub fn type_is_f32<T>(value: T) -> bool where T: Any {
        TypeId::of::<f32>() == value.type_id()
    }

    pub fn type_is_f64<T>(value: T) -> bool where T: Any {
        TypeId::of::<f64>() == value.type_id()
    }

    pub fn type_is_str<T>(value: T) -> bool where T: Any {
        TypeId::of::<&str>() == value.type_id()
    }

    pub fn type_is_bool<T>(value: T) -> bool where T: Any {
        TypeId::of::<bool>() == value.type_id()
    }

    pub fn type_is_char<T>(value: T) -> bool where T: Any {
        TypeId::of::<char>() == value.type_id()
    }

    pub fn type_is_vec<T>(value: T) -> bool
        where
            T: Any,
            V: Any
    {
        TypeId::of::<Vec<V>>() == value.type_id()
    }

    pub fn type_is_neo_map<T>(value: T) -> bool where T: Any {
        TypeId::of::<NeoMap>() == value.type_id()
    }

    pub fn type_is_value<T>(value: T) -> bool where T: Any {
        TypeId::of::<Value>() == value.type_id()
    }

    pub fn type_is_string<T>(value: T) -> bool where T: Any {
        TypeId::of::<String>() == value.type_id()
    }

    pub fn type_is_equal<L, R>(left: L, right: R) -> bool
        where
            L: Any,
            R: Any
    {
        left.type_id() == right.type_id()
    }
}
