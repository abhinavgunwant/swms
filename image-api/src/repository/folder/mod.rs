pub mod db;

use crate::{ db::DBError, model::folder::Folder };

pub trait FolderRepository {
    fn get(&self, id: u32) -> Result<Folder, DBError>;
    fn get_from_slug(&self, slug: String) -> Result<Folder, DBError>;
    fn get_all_from_project(&self, project_id: u32) -> Result<Vec<Folder>, DBError>;

    /// Returns subfolders of a folder from it's id.
    ///
    /// # Arguments
    /// * `folder_id` - The folder id.
    fn get_from_folder(&self, folder_id: u32) -> Result<Vec<Folder>, DBError>;

    /// Returns subfolders of a folder from it's slug.
    ///
    /// # Arguments
    /// * `folder_slug` - The folder slug.
    /// * `all` - Whether to fetch all the folders inside this folder.
    fn get_from_folder_slug(&self, folder_slug: String, all: bool)
        -> Result<Vec<Folder>, DBError>;

    /// Returns folders from a project slug.
    /// 
    /// # Arguments
    /// 
    /// * `project_slug` - The project slug.
    /// * `all` -  Whether to fetch all the folders inside a project.
    /// 
    fn get_from_project_slug(&self, project_slug: String, all: bool)
        -> Result<Vec<Folder>, DBError>;

    fn add(&self, folder: Folder) -> Result<String, String>;

    /**
     * Validates if a rendition with the provided slug doesn't exists for a
     * given image.
     *
     * Used for providing real-time validation while the admin is typing the
     * project name (or project slug) in "New Project" screen.
     *
     * `slug`: The slug provided (should be `lowercase`).
     *
     * Returns true if a project with the supplied slug doesn't exist.
     */
    fn is_valid_new_slug(&self, slug: String) -> Result<bool, DBError>;

    /**
     * Validates if a rendition with the provided slug exists for image.
     *
     * Behaves exactly opposite to `validate_new_project_slug`.
     */
    fn is_valid_slug(&self, project_id: u32, folder_id: u32, slug: String) ->
        Result<Option<u32>, DBError>;

    fn update(&self, folder: Folder) -> Result<String, String>;
    fn remove(&self, folder: Folder) -> Result<String, String>;
    fn remove_item(&self, id: u32) -> Result<String, String>;
}

//pub fn get_folder_repository() -> impl FolderRepository {
//    let dctxt = get_db_context();
//
//    match dctxt.dbimpl {
//        DBImpl::MYSQL => {
//            MySQLFolderRepository { connection: mysql::Pool::new("").expect("lol").get_conn().unwrap() }
//        }
//    }
//}

