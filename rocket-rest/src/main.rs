#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

#[path = "./users/mod.rs"]
mod users;

use crate::users::user_service;
use crate::users::user_model::User;

#[cfg(test)]
mod tests;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn list_users() -> Json<Vec<User>> {
    Json(user_service::list_users())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, list_users])
}
