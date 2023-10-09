use sqlx::FromRow;
use chrono::prelude::*;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, FromRow)]
#[allow(non_snake_case)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub phone: String,
    pub job_title: String,
    pub password: String,
    pub address_id: i64,
    pub created: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };


    let query_result = sqlx::query_as!(
        User,
        "SELECT * FROM user_auth.users ORDER by id"
    ).fetch_all(&pool)
        .await;

    println!("{:?}", query_result.unwrap())
}
