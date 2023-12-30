mod models;
mod schema;
mod repositories;
use rocket_db_pools::{Connection, Database};


#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[rocket::get("/rustaceans")]
fn get_rustaceans(db: Connection<DbConn>) {

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