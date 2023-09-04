#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};

#[path = "./users/mod.rs"]
mod users;

use crate::users::user_model::User;
use crate::users::user_service;

#[cfg(test)]
mod tests;

#[openapi(tag = "Health")]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[openapi(tag = "Users")]
#[get("/users")]
fn list_users() -> Json<Vec<User>> {
    Json(user_service::list_users())
}

//#[launch]
//fn rocket() -> _ {
//    rocket::build().mount("/", routes![index, list_users])
//
//

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![index, list_users])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}
