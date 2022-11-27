pub mod db;

use crate::{
    db::{ DBError, DBImpl, get_db_context },
    model::{
        user::User, user_search::UserSearch, user_permissions::UserPermissions,
    },
};
use db::mysql::MySQLUserRepository;

pub trait UserRepository {
    fn get(&self, id: u32) -> Result<User, DBError>;
    fn get_from_login_id(&self, login_id: String) -> Result<User, DBError>;
    fn get_password_for_login_id(&self, login_id: String)
        -> Result<String, DBError>;
    fn get_permissions(&self, login_id: String)
        -> Result<UserPermissions, String>;
    fn get_all(&self) -> Result<Vec<User>, DBError>;
    fn get_all_paged(&self, page: u32, page_length: u32)
        -> Result<Vec<User>, DBError>;
    fn search_from_name(&self, name_query: String, page_length: u32)
        -> Result<Vec<UserSearch>, DBError>;
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
