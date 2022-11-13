use serde::Serialize;
use chrono::{ DateTime, Utc };

#[derive(Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub login_id: String,
    #[serde(skip_serializing)]
    pub password: String, // Should be hidden for privacy when serializing :)
    pub email: String,
    pub user_role: u8,
    pub created_by: u32,
    pub modified_by: u32,
    pub created_on: DateTime<Utc>,
    pub modified_on: DateTime<Utc>,
    pub last_login_on: DateTime<Utc>,
}
