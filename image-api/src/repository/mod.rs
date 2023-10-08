pub mod image;
pub mod user;
pub mod item;
mod metadata;
pub mod rendition;
pub mod project;
pub mod folder;
pub mod role;

use mysql::Pool;

use crate::server::db::{ DBError, mysql_to_db_error };
use self::role::{ RoleRepository, db::mysql::MySQLRoleRepository };

pub trait Repository {
    fn get_role_repo(&self) -> Result<Box::<dyn RoleRepository>, DBError>;
}

#[derive(Clone)]
pub struct MySQLRepository {
    connection_pool: Pool,
}

impl Repository for MySQLRepository {
    fn get_role_repo(&self) -> Result<Box::<dyn RoleRepository>, DBError> {
        match self.connection_pool.get_conn() {
            Ok(connection) => Ok(Box::new(MySQLRoleRepository { connection })),
            Err(e) => Err(
                mysql_to_db_error("Error while creating connection", e)
            ),
        }
    }
}

impl MySQLRepository {
    pub fn new(connection_str: &str) -> Result<Self, DBError> {
        match Pool::new(connection_str) {
            Ok(pool) => Ok(MySQLRepository { connection_pool: pool }),
            Err(e) => Err(mysql_to_db_error("Error while creating pool", e)),
        }
    }
}

