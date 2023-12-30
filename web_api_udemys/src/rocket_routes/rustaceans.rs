use crate::DbConn;
use crate::models::{NewRustacean, Rustacean};
use crate::repositories::RustaceanRepository;
use rocket::serde::json::{json, Json};
use rocket_db_pools::{Connection};
use rocket::serde::json::Value;
use rocket::response::status::{Custom, NoContent};
use rocket::http::Status;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    const LIMIT: i64= 100;
    RustaceanRepository::find_multiple(&mut db, LIMIT).await
        .map(|r| json!(r))
        .map_err(|_| Custom(Status::InternalServerError , json!("err")))
}

#[rocket::get("/rustacean/<id>")]
pub async fn view_rustaceans(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find(&mut db, id).await
        .map(|r| json!(r))
        .map_err(|_| Custom(Status::InternalServerError , json!("err")))
}


#[rocket::post("/rustaceans", format="json", data= "<new_rustacean>")]
pub async fn create_rustaceans(mut db: Connection<DbConn>, new_rustacean: Json<NewRustacean>) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut db, new_rustacean.into_inner()).await
        .map(|r| Custom(Status::Created, json!(r)))
        .map_err(|_| Custom(Status::InternalServerError , json!("err")))
}


#[rocket::put("/rustaceans/<id>", format="json", data= "<rustacean>")]
pub async fn update_rustaceans(mut db: Connection<DbConn>, id: i32, rustacean: Json<Rustacean>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::update(&mut db, id, rustacean.into_inner()).await
        .map(|r| json!(r))
        .map_err(|_| Custom(Status::InternalServerError , json!("err")))
}

#[rocket::delete("/rustacean/<id>")]
pub async fn delete_rustaceans(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    RustaceanRepository::delete(&mut db, id).await
    .map(|_| NoContent)
    .map_err(|_| Custom(Status::InternalServerError , json!("err")))
}
