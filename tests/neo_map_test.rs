use neo_rust::NeoMap;
use neo_rust::Put;

pub struct MyClass {
    name: String
}

#[test]
pub fn put_base_test1() {
    let neo_map = NeoMap::new();

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

    neo_map.put("i8", i8);
    neo_map.put("i16", i16);
    neo_map.put("i32", i32);
    neo_map.put("i64", i64);

    neo_map.put("u8", u8);
    neo_map.put("u16", u16);
    neo_map.put("u32", u32);
    neo_map.put("u64", u64);

    neo_map.put("f32", f32);
    neo_map.put("f64", f64);

    neo_map.put("true", true);
    neo_map.put("false", false);

    assert_eq!(i8, neo_map.get_i8("i8").unwrap());
    assert_eq!(i16, neo_map.get_i16("i16").unwrap());
    assert_eq!(i32, neo_map.get_i32("i32").unwrap());
    assert_eq!(i64, neo_map.get_i64("i64").unwrap());

    assert_eq!(u8, neo_map.get_u8("u8").unwrap());
    assert_eq!(u16, neo_map.get_u16("u16").unwrap());
    assert_eq!(u32, neo_map.get_u32("u32").unwrap());
    assert_eq!(u64, neo_map.get_u64("u64").unwrap());

    assert_eq!(f32, neo_map.get_f32("f32").unwrap());
    assert_eq!(f64, neo_map.get_f64("f64").unwrap());

    assert_eq!(true, neo_map.get_bool("true").unwrap());
    assert_eq!(false, neo_map.get_bool("false").unwrap());
}

// #[test]
// pub fn put_base_test2() {
//     let neo_map = NeoMap::new();
//     let mut v = Vec::new();
//     v.push(12);
//     v.push(13);
//     neo_map.put("vec1", v);
//
//     let mut v = Vec::new();
//     v.push(MyClass{name: String::from("v1")});
//     v.push(MyClass{name: String::from("v3")});
//     neo_map.put("vec2", v);
//
//     neo_map.put("neo_map", NeoMap::new().put("k1", "v1").put("k2", "v2"));
//     neo_map.put("type", MyClass { name: String::from("ok") });
//
//     println!("{}", neo_map.get_string("key1").unwrap());
// }
//
// #[test]
// pub fn put_base_test3() {
//     let inner_map = NeoMap::new();
//     inner_map.put("k1", "v1");
//     inner_map.put("k2", "v2");
//
//     let neo_map = NeoMap::new();
//     neo_map.put("outer", inner_map);
//     neo_map.put("simple", "sdf");
//
//     println!("{}", neo_map.get("simple").unwrap())
// }
