use rocket::serde::Serialize;
use serde::Deserialize;
use chrono::prelude::*;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Clone, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: u8,
    pub name: String,
    pub created: DateTime<FixedOffset>,
    pub active: bool
}
