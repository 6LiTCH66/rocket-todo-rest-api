mod api;
mod models;
mod repository;

#[macro_use] extern crate rocket;

use rocket::{Build, get, http::Status, Rocket, serde::json::Json};
use std::error::Error;
use api::todo_api::{create_todo, get_todo, get_all_todos, update_todo, delete_todo};

#[launch]
fn rocket() -> _{
    let db = repository::mongodb_repo::MongoRepo::init().unwrap();
    rocket::build().manage(db)
        .mount("/", routes![create_todo])
        .mount("/", routes![get_todo])
        .mount("/", routes![get_all_todos])
        .mount("/", routes![update_todo])
        .mount("/", routes![delete_todo])
}
