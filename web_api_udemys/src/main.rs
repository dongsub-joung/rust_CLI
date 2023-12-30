mod models;
mod schema;
mod repositories;

use repositories::RustaceanRepository;
use rocket::serde::json::json;
use rocket_db_pools::{Connection, Database};
use rocket::serde::json::Value;
use rocket::response::status::Custom;
use rocket::http::Status;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[rocket::get("/rustaceans")]
async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    const LIMIT: i64= 100;
    RustaceanRepository::find_multiple(&mut db, LIMIT).await
        .map(|r| json!(r))
        .map_err(|_| Custom(Status::InternalServerError , json!("err")))
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            get_rustaceans
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}