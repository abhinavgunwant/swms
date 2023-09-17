pub mod db;

use std::result::Result;
use crate::{
    db::{ DBError, DBImpl, get_db_context },
    model::role::Role,
};
use db::mysql::MySQLRoleRepository;

pub trait RoleRepository {
    fn get(&self, id: u8) -> Result<Role, DBError>;
    fn get_all(&self) -> Result<Vec<Role>, DBError>;
    fn add(&self, role: Role) -> Result<String, String>;
    fn update(&self, role: Role) -> Result<String, String>;
    fn remove(&self, role: Role) -> Result<String, String>;
    fn remove_item(&self, id: u32) -> Result<String, String>;
}

pub fn get_role_repository() -> impl RoleRepository {
    let dctxt = get_db_context();

    match dctxt.dbimpl {
        DBImpl::MYSQL => {
            MySQLRoleRepository {}
        }
    }
}

