use chrono::DateTime;

use super::item::Item;

struct User {
    id: u32,
    name: String,
    login_id: String,
    email: String,
    user_role: u8,
    created_on: DateTime,
    last_login_on: DateTime
}

impl Item for User {
    fn id(&self) -> u32 {
        return self.id;
    }

    fn slug(&self) -> String {
        return String::to_string("Test slug");
    }

    fn created_on(&self) -> DateTime {
        return DateTime;
    }

    fn created_by(&self) -> u16 {
        return 0;
    }

    fn modified_on(&self) -> DateTime {
        return DateTime;
    }

    fn modified_by(&self) -> u16 {
        return 0;
    }
}

