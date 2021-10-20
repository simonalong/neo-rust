use neo_rust::NeoMap;
use neo_rust::Put;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
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

    let str = "str";

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

    neo_map.put("str", str);

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

    assert_eq!(str, neo_map.get_string("str").unwrap());
}

#[test]
pub fn put_base_test2() {
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

    let true_str = true;
    let false_str = false;

    neo_map.put("i8", &i8);
    neo_map.put("i16", &i16);
    neo_map.put("i32", &i32);
    neo_map.put("i64", &i64);

    neo_map.put("u8", &u8);
    neo_map.put("u16", &u16);
    neo_map.put("u32", &u32);
    neo_map.put("u64", &u64);

    neo_map.put("f32", &f32);
    neo_map.put("f64", &f64);

    neo_map.put("true", &true_str);
    neo_map.put("false", &false_str);

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

#[test]
pub fn put_base_test3() {
    let neo_map = NeoMap::new();
    let mut v = Vec::new();
    v.push(12);
    v.push(13);
    neo_map.put("vec1", &v);

    let result = neo_map.get_vec("vec1").unwrap();
    let mut re_v = Vec::new();
    for x in result {
        re_v.push(x.as_i64().unwrap() as i32)
    }
    assert_eq!(v, re_v);


    // todo 无法执行
    // let mut v = Vec::new();
    // v.push(MyClass{name: String::from("v1")});
    // v.push(MyClass{name: String::from("v3")});
    // neo_map.put("vec2", v);
    //
    // let result = neo_map.get_vec("vec2").unwrap();
    // let mut re_v = Vec::new();
    // for x in result {
    //     let d:MyClass = serde_json::from_value(x).unwrap();
    //     re_v.push(d)
    // }
    // assert_eq!(v, re_v);

    // neo_map.put("neo_map", NeoMap::new().put("k1", "v1").put("k2", "v2"));
    // neo_map.put("type", MyClass { name: String::from("ok") });


}
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
