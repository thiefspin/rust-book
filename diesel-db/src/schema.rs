// @generated automatically by Diesel CLI.

pub mod user_auth {
    diesel::table! {
        user_auth.roles (id) {
            id -> Int8,
            #[max_length = 255]
            name -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        user_auth.users (id) {
            id -> Int8,
            #[max_length = 40]
            email -> Varchar,
            #[max_length = 40]
            name -> Varchar,
            #[max_length = 40]
            surname -> Varchar,
            #[max_length = 15]
            phone -> Varchar,
            #[max_length = 40]
            job_title -> Varchar,
            password -> Varchar,
            address_id -> Int8,
            created -> Timestamp,
            last_login -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        user_auth.users_roles (id) {
            id -> Int8,
            role_id -> Int8,
            user_id -> Int8,
        }
    }

    diesel::joinable!(users_roles -> roles (role_id));
    diesel::joinable!(users_roles -> users (user_id));

    diesel::allow_tables_to_appear_in_same_query!(
        roles,
        users,
        users_roles,
    );
}
