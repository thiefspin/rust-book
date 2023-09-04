use crate::users::user_model::User;
use chrono::prelude::*;

pub fn list_users() -> Vec<User> {
    return load_data();
}

fn load_data() -> Vec<User> {
    return [
        User {
            id: 1,
            name: "Frikkie".to_string(),
            created: sast_date_time(),
            active: true,
        },
        User {
            id: 2,
            name: "Pietie".to_string(),
            created: sast_date_time(),
            active: true,
        },
    ].to_vec();
}

fn sast_date_time() -> DateTime<FixedOffset> {
    let utc: DateTime<Utc> = Utc::now();
    let offset = FixedOffset::east_opt(2*3600).unwrap();
    let sast: DateTime<FixedOffset> = DateTime::with_timezone(&utc, &offset);
    return sast;
}
