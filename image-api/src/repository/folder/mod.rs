pub mod db;

use crate::{ server::db::DBError, model::folder::Folder };

pub trait FolderRepository {
    /// Finds out if the folder repository exists along wth verifying
    /// all the columns
    fn verify(&mut self) -> Result<(), DBError>;

    fn get(&mut self, id: u32) -> Result<Folder, DBError>;
    fn get_from_slug(&mut self, slug: String) -> Result<Folder, DBError>;
    fn get_all_from_project(&mut self, project_id: u32) -> Result<Vec<Folder>, DBError>;

    /// Returns subfolders of a folder from it's id.
    ///
    /// # Arguments
    /// * `folder_id` - The folder id.
    fn get_from_folder(&mut self, folder_id: u32) -> Result<Vec<Folder>, DBError>;

    /// Returns subfolders of a folder from it's slug.
    ///
    /// # Arguments
    /// * `folder_slug` - The folder slug.
    /// * `all` - Whether to fetch all the folders inside this folder.
    fn get_from_folder_slug(&mut self, folder_slug: String, all: bool)
        -> Result<Vec<Folder>, DBError>;

    /// Returns folders from a project slug.
    /// 
    /// # Arguments
    /// 
    /// * `project_slug` - The project slug.
    /// * `all` -  Whether to fetch all the folders inside a project.
    /// 
    fn get_from_project_slug(&mut self, project_slug: String, all: bool)
        -> Result<Vec<Folder>, DBError>;

    fn add(&mut self, folder: Folder) -> Result<String, String>;

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
    fn is_valid_new_slug(&mut self, slug: String) -> Result<bool, DBError>;

    /**
     * Validates if a rendition with the provided slug exists for image.
     *
     * Behaves exactly opposite to `validate_new_project_slug`.
     */
    fn is_valid_slug(&mut self, project_id: u32, folder_id: u32, slug: String) ->
        Result<Option<u32>, DBError>;

    fn update(&mut self, folder: Folder) -> Result<String, String>;
    fn remove(&mut self, folder: Folder) -> Result<String, String>;
    fn remove_item(&mut self, id: u32) -> Result<String, String>;
}

