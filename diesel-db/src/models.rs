use diesel::prelude::*;
use chrono::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_auth::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
    pub last_login: Option<NaiveDateTime>
}
