use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: u8,
    pub name: String,
    pub active: bool,
}