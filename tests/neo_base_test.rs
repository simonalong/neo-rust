use neo_rust::{Neo, NeoMap};
use sqlx::mysql::{MySqlPoolOptions, MySqlQueryResult, MySqlRow};


#[test]
pub fn connect_test() {
    // let pool = MySqlPoolOptions::new()
    //     .max_connections(5)
    //     .connect("mysql://neo_test:neo@Test123@localhost:3306/demo1").await?;

    let neo = Neo::connect("mysql://neo_test:neo@Test123@localhost:3306/demo1");

    // 新增
    let data = NeoMap::of("name", "name1", "group", "group1");
    neo.insert("demo1", data);

    // 删除

    // 修改

    // 查询：一行

    // 查询：查询多行

    // 查询：一个值

    // 查询：一列的多个值

    // 查询：分页

    // 查询：是否存在

    // 执行sql

    // 事务

    // 批量插入

    // 批量更新
}
