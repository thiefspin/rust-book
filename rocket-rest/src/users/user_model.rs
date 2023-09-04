use rocket::serde::Serialize;
use serde::Deserialize;
use chrono::prelude::*;

#[derive(Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: u8,
    pub name: String,
    pub created: DateTime<FixedOffset>,
    pub active: bool
}
