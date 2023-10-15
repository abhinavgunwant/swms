pub mod db;

use crate::{
    server::db::DBError, model::{
        user::User, user_search::UserSearch, user_permissions::UserPermissions,
    },
};

pub trait UserRepository {
    fn get(&mut self, id: u32) -> Result<User, DBError>;
    fn get_from_login_id(&mut self, login_id: String) -> Result<User, DBError>;
    fn get_password_for_login_id(&mut self, login_id: String)
        -> Result<String, DBError>;
    fn get_permissions(&mut self, login_id: String)
        -> Result<UserPermissions, String>;
    fn get_all(&mut self) -> Result<Vec<User>, DBError>;
    fn get_all_paged(&mut self, page: u32, page_length: u32)
        -> Result<Vec<User>, DBError>;
    fn search_from_name(&mut self, name_query: String, page_length: u32)
        -> Result<Vec<UserSearch>, DBError>;
    fn add(&mut self, user: User) -> Result<u32, String>;
    fn update(&mut self, user: User) -> Result<(), String>;
    fn remove(&self, id: User);
    fn remove_item(&self, id: u32);
}

