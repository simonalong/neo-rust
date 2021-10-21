use neo_rust::ObjectUtil;

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

    assert_eq!(true, ObjectUtil::type_is_i8(i8));
    assert_eq!(true, ObjectUtil::type_is_i16(i16));
    assert_eq!(true, ObjectUtil::type_is_i32(i16));
    assert_eq!(true, ObjectUtil::type_is_i64(i16));
    assert_eq!(true, ObjectUtil::type_is_isize(isize));
    assert_eq!(true, ObjectUtil::type_is_u8(i16));
    assert_eq!(true, ObjectUtil::type_is_u16(i16));
    assert_eq!(true, ObjectUtil::type_is_u32(i16));
    assert_eq!(true, ObjectUtil::type_is_u64(i16));
    assert_eq!(true, ObjectUtil::type_is_usize(i16));
    assert_eq!(true, ObjectUtil::type_is_f32(i16));
    assert_eq!(true, ObjectUtil::type_is_f64(i16));
    assert_eq!(true, ObjectUtil::type_is_str(i16));
    assert_eq!(true, ObjectUtil::type_is_bool(i16));
    assert_eq!(true, ObjectUtil::type_is_char(c));
    assert_eq!(false, ObjectUtil::type_is_equal(i8, i16));
}
