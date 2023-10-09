use std::io::ErrorKind::NotFound;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::users::user_model::User;
use crate::users::user_service;

#[path = "../users/mod.rs"]
mod users;

#[openapi(tag = "Users")]
#[get("/")]
pub fn list_users() -> Json<Vec<User>> {
    Json(user_service::list_users())
}

#[openapi(tag = "Users")]
#[get("/<id>")]
pub fn get_user(id: u64) -> Option<Json<User>> {
    return user_service::list_users().iter().find(|u| u.id == id).map(|u| Json(u.to_owned()));
}