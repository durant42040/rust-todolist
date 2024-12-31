use sqlx::MySqlPool;
use std::env::var;

#[derive(Clone)]
pub struct Db {
    pub mysql_pool: MySqlPool,
}

impl Db {
    pub async fn init() -> Db {
        let database_url = var("DATABASE_URL").expect("DATABASE_URL is not set");

        let mysql_pool = MySqlPool::connect(&database_url)
            .await
            .expect("Failed to connect to MySQL");
        println!("Connected to MySQL");

        Self { mysql_pool }
    }
}
