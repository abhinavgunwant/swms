pub mod db;

use crate::{ db::DBError, model::role::Role, };

pub trait RoleRepository {
    fn get(&mut self, id: u8) -> Result<Role, DBError>;
    fn get_all(&mut self) -> Result<Vec<Role>, DBError>;
    fn add(&mut self, role: Role) -> Result<String, String>;
    fn update(&mut self, role: Role) -> Result<String, String>;
    fn remove(&mut self, role: Role) -> Result<String, String>;
    fn remove_item(&mut self, id: u32) -> Result<String, String>;
}

