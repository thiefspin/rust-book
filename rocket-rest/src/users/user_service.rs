use crate::users::user_model::User;

pub fn list_users() -> Vec<User> {
    return load_data();
}

fn load_data() -> Vec<User> {
    return [
        User {
            id: 1,
            name: "Frikkie".to_string(),
            active: true,
        },
        User {
            id: 2,
            name: "Pietie".to_string(),
            active: true,
        },
    ].to_vec();
}