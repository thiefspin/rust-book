mod models;
mod schema;

use diesel::prelude::*;
use crate::models::User;
use crate::schema::user_auth::users::dsl::users;
use dotenvy::dotenv;
use std::env;

fn main() {
    use self::schema::user_auth::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(id.eq(1))
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for user in results {
        println!("{}", user.name);
        println!("-----------\n");
        println!("{}", user.surname);
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
