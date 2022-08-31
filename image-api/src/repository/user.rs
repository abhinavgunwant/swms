use chrono::{ DateTime, Utc };

use super::item::Item;

struct User {
    id: u32,
    name: String,
    login_id: String,
    email: String,
    user_role: u8,
    created_on: DateTime<Utc>,
    last_login_on: DateTime<Utc>
}

impl Item for User {
    fn id(&self) -> u32 {
        return self.id;
    }

    fn slug(&self) -> String {
        return "Test slug".to_string();
    }

    fn created_on(&self) -> DateTime<Utc> {
        return Utc::now();
    }

    fn created_by(&self) -> u16 {
        return 0;
    }

    fn modified_on(&self) -> DateTime<Utc> {
        return Utc::now();
    }

    fn modified_by(&self) -> u16 {
        return 0;
    }
}

