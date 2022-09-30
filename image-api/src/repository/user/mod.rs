pub mod db;

use serde::Serialize;
use chrono::{ DateTime, Utc };
use crate::db::{ DBError, DBImpl, get_db_context };
use db::mysql::MySQLUserRepository;

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

pub trait UserRepository {
    fn get(&self, id: u32) -> Result<User, DBError>;
    fn get_from_login_id(&self, login_id: String) -> Result<User, DBError>;
    fn get_password_for_login_id(&self, login_id: String) -> Result<String, DBError>;
    fn get_all(&self) -> Vec::<User>;
    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<User>;
    fn add(&self, user: User);
    fn update(&self, user: User);
    fn remove(&self, id: User);
    fn remove_item(&self, id: u32);
}

pub fn get_user_repository() -> impl UserRepository {
    let dctxt = get_db_context();

    match dctxt.dbimpl {
        DBImpl::MYSQL => {
            MySQLUserRepository {}
        }
    }
}
