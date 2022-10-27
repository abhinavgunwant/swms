mod db;

use chrono::{ DateTime, Utc };
use crate::db::{ DBError, DBImpl, get_db_context };
use serde::{ Serialize, Deserialize };
use serde_json;
use db::mysql::MySQLFolderRepository;

#[derive(Serialize, Deserialize)]
pub struct Folder {
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub project_id: u16,
    pub description: String,
    pub parent_folder_id: u32,
    pub created_by: u32,
    pub modified_by: u32,
    pub created_on: DateTime<Utc>,
    pub modified_on: DateTime<Utc>
}

pub trait FolderRepository {
    fn get(&self, id: u32) -> Result<Folder, DBError>;
    fn get_from_slug(&self, slug: String) -> Result<Folder, DBError>;
    fn add(&self, folder: Folder);
    fn remove(&self, id: u32);
}

pub fn get_folder_repository() -> impl FolderRepository {
    let dctxt = get_db_context();

    match dctxt.dbimpl {
        DBImpl::MYSQL => {
            MySQLFolderRepository {}
        }
    }
}
