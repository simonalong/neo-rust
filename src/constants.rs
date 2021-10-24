#[macro_use]
use lazy_static;

pub static MYSQL: &str = "mysql";
pub static POSTGRES: &str = "postgres";
pub static SQLITE: &str = "sqlite";
pub static MSSQL: &str = "mssql";
pub static DOM: &str = "`";


lazy_static! {
    // mysql类型对应rust的默认类型
    static ref MYSQL_TYPE_RUST: HashMap<&str, &str> = {
        let mut map = HashMap::new();
        map.insert(18, "hury".to_owned());
        map
    };
    // rust类型对应mysql的默认类型
    static ref RUST_TYPE_MYSQL: HashMap<&str, &str> = {
        let mut map = HashMap::new();
        map.insert(18, "hury".to_owned());
        map
    };
}
