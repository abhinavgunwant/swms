use chrono::DateTime;

struct User {
    id: u16,
    name: String,
    login_id: String,
    email: String,
    user_role: u8,
    created_on: DateTime,
    last_login_on: DateTime
}

impl Item for User {
    fn getId(&self) -> u32 {
        return self.id;
    }

    fn getSlug(&self) -> String {
        return String::to_string("Test slug");
    }

    fn getCreatedOn(&self) -> DateTime {
        return DateTime;
    }

    fn getCreatedBy(&self) -> u16 {
        return 0;
    }

    fn getModifiedOn(&self) -> DateTime {
        return DateTime;
    }

    fn getModifiedBy(&self) -> u16 {
        return 0;
    }
}

