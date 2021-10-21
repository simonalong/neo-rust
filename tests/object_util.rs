use neo_rust::{ObjectUtil, NeoMap};
use serde_json::Value;

#[test]
pub fn test1() {
    let i8:i8 = 0;
    let i16:i16 = 0;
    let i32:i32 = 0;
    let i64:i64 = 0;

    let u8:u8 = 0;
    let u16:u16 = 0;
    let u32:u32 = 0;
    let u64:u64 = 0;

    let f32:f32 = 0.0;
    let f64:f64 = 0.1;

    let str = "str";
    let true_str = true;
    let false_str = false;
    let c:char = 'c';

    assert_eq!(true, ObjectUtil::type_is_base(i8));
    assert_eq!(true, ObjectUtil::type_is_base(i16));
}

#[test]
pub fn test2() {
    let i8:i8 = 0;
    let i16:i16 = 0;
    let i32:i32 = 0;
    let i64:i64 = 0;
    let isize:isize = 0;

    let u8:u8 = 0;
    let u16:u16 = 0;
    let u32:u32 = 0;
    let u64:u64 = 0;
    let usize:usize = 0;

    let f32:f32 = 0.0;
    let f64:f64 = 0.1;

    let str = "str";
    let true_str = true;
    let false_str = false;
    let c:char = 'c';
    let neo_map:NeoMap = NeoMap::new();
    let value:Value = Value::from("str");
    let string:String = String::from("str");

    assert_eq!(true, ObjectUtil::type_is_i8(i8));
    assert_eq!(true, ObjectUtil::type_is_i16(i16));
    assert_eq!(true, ObjectUtil::type_is_i32(i32));
    assert_eq!(true, ObjectUtil::type_is_i64(i64));
    assert_eq!(true, ObjectUtil::type_is_isize(isize));
    assert_eq!(true, ObjectUtil::type_is_u8(u8));
    assert_eq!(true, ObjectUtil::type_is_u16(u16));
    assert_eq!(true, ObjectUtil::type_is_u32(u32));
    assert_eq!(true, ObjectUtil::type_is_u64(u64));
    assert_eq!(true, ObjectUtil::type_is_usize(usize));
    assert_eq!(true, ObjectUtil::type_is_f32(f32));
    assert_eq!(true, ObjectUtil::type_is_f64(f64));
    assert_eq!(true, ObjectUtil::type_is_str(str));
    assert_eq!(true, ObjectUtil::type_is_bool(true));
    assert_eq!(true, ObjectUtil::type_is_char(c));
    assert_eq!(true, ObjectUtil::type_is_neo_map(neo_map));
    assert_eq!(true, ObjectUtil::type_is_value(value));
    assert_eq!(true, ObjectUtil::type_is_string(string));
    assert_eq!(false, ObjectUtil::type_is_equal(i8, i16));
}
