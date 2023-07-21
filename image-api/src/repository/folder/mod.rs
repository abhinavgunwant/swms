mod db;

use crate::db::{ DBError, DBImpl, get_db_context };
use crate::model::folder::Folder;
use db::mysql::MySQLFolderRepository;

pub trait FolderRepository {
    fn get(&self, id: u32) -> Result<Folder, DBError>;
    fn get_from_slug(&self, slug: String) -> Result<Folder, DBError>;
    fn get_all_from_project(&self, project_id: u32) -> Result<Vec<Folder>, DBError>;
    fn get_all_from_project_slug(&self, project_slug: String)
        -> Result<Vec<Folder>, DBError>;
    fn get_all_from_folder_slug(&self, folder_slug: String)
        -> Result<Vec<Folder>, DBError>;
    fn add(&self, folder: Folder) -> Result<String, String>;
    fn update(&self, folder: Folder) -> Result<String, String>;
    fn remove(&self, folder: Folder) -> Result<String, String>;
    fn remove_item(&self, id: u32) -> Result<String, String>;
}

pub fn get_folder_repository() -> impl FolderRepository {
    let dctxt = get_db_context();

    match dctxt.dbimpl {
        DBImpl::MYSQL => {
            MySQLFolderRepository {}
        }
    }
}

