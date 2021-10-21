use sqlx::{Database, Pool, MySql};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;

pub struct Neo
{
    pub connect_pool: Pool<MySql>
}

impl Neo
{
    pub async fn connect(uri: &str) -> Neo {
        if uri == "" {
            panic!("uri cannot empty")
        }

        if uri.to_string().starts_with("mysql") {
            return Neo{connect_pool: MySqlPoolOptions::new().connect(uri).await.unwrap()}
        // } else if uri.to_string().starts_with("postgres") {
        //     return &Neo{connect_pool: PgPoolOptions::new().connect(uri).await?}
        // } else if uri.to_string().starts_with("sqlite") {
        //     return &Neo{connect_pool: SqlitePoolOptions::new().connect(uri).await?}
        } else {
            panic!("not support database: {}", uri);
        }
    }
}

#[test]
pub fn test_connect() {
    let neo = Neo::connect("mysql://neo_test:neo@Test123@localhost:3306/demo1");
}
