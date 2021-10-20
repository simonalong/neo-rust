struct MyClass {
    name: String
}

trait Tem<T> {
    fn test(&self, key: &str, v: T);
}

impl Tem<String> for MyClass {
    fn test(&self, key: &str, v: String) {
        println!("haode1")
    }
}

impl Tem<&str> for MyClass {
    fn test(&self, key: &str, v: &str) {
        println!("haode2")
    }
}

#[test]
pub fn test1() {
    let m = MyClass { name: String::from("") };
    m.test("xx", String::from("sd"));
    m.test("xx", "sddf");
}
