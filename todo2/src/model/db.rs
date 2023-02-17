use sqlx::{Pool, postgres};

pub type Db= pool<Postgres>;

async fn new_db_pool(host &str, db: &str, user: &str, pwd: &str, max_con: u32 -> Result<Db, sqlx::Error> {
    let con_string= format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connection(max_con)
        .connect_timeout(Duration::from::millis(500))
        .connect(&con_string)
        .await
}
