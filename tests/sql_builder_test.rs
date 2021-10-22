use neo_rust::{SqlBuilder, Neo};

#[tokio::test]
async fn test1() {
    let data = SqlBuilder::to_db_field(String::from("name"));
    println!("{}", data);
}



