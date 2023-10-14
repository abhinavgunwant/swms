pub mod image;
pub mod user;
pub mod item;
pub mod rendition;
pub mod project;
pub mod folder;
pub mod role;

use mysql::Pool;

use crate::server::db::{ DBError, mysql_to_db_error };
use self::{
    folder::{ FolderRepository, db::mysql::MySQLFolderRepository },
    role::{ RoleRepository, db::mysql::MySQLRoleRepository },
    image::{ ImageRepository, db::mysql::MySQLImageRepository },
    project::{ ProjectRepository, db::mysql::MySQLProjectRepository },
    rendition::{ RenditionRepository, db::mysql::MySQLRenditionRepository },
};

pub trait Repository {
    fn get_role_repo(&self) -> Result<Box::<dyn RoleRepository>, DBError>;
    fn get_folder_repo(&self) -> Result<Box::<dyn FolderRepository>, DBError>;
    fn get_image_repo(&self) -> Result<Box::<dyn ImageRepository>, DBError>;
    fn get_project_repo(&self) -> Result<Box::<dyn ProjectRepository>, DBError>;
    fn get_rendition_repo(&self) -> Result<Box::<dyn RenditionRepository>, DBError>;
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

    fn get_folder_repo(&self) -> Result<Box::<dyn FolderRepository>, DBError> {
        match self.connection_pool.get_conn() {
            Ok(connection) => Ok(Box::new(MySQLFolderRepository { connection })),
            Err(e) => Err(
                mysql_to_db_error("Error while creating connection", e)
            ),
        }
    }

    fn get_image_repo(&self) -> Result<Box::<dyn ImageRepository>, DBError> {
        match self.connection_pool.get_conn() {
            Ok(connection) => Ok(Box::new(MySQLImageRepository { connection })),
            Err(e) => Err(
                mysql_to_db_error("Error while creating connection", e)
            ),
        }
    }

    fn get_project_repo(&self) -> Result<Box::<dyn ProjectRepository>, DBError> {
        match self.connection_pool.get_conn() {
            Ok(connection) => Ok(Box::new(MySQLProjectRepository { connection })),
            Err(e) => Err(
                mysql_to_db_error("Error while creating connection", e)
            ),
        }
    }

    fn get_rendition_repo(&self) -> Result<Box::<dyn RenditionRepository>, DBError> {
        match self.connection_pool.get_conn() {
            Ok(connection) => Ok(Box::new(MySQLRenditionRepository { connection })),
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

