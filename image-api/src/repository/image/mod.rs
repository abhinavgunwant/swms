pub mod db;

use crate::db::{ DBImpl, get_db_context, DBError };
use crate::model::{ encoding::Encoding, image::Image };
use db::mysql::MySQLImageRepository;

pub trait ImageRepository {
    fn get(&self, id: u32) -> Result<Image, DBError>;
    fn get_from_slug(&self, slug: &str) -> Result<Image, DBError>;
    fn get_all(&self) -> Result<Vec<Image>, DBError>;
    fn get_all_from_project(&self, project_id: u32)
        -> Result<Vec::<Image>, DBError>;
    fn get_all_paged(&self, page: u32, page_length: u32)
        -> Result<Vec<Image>, DBError>;

    /// Returns images inside a folder from folder slug.
    /// 
    /// # Arguments
    /// 
    /// * `folder_slug` - The folder slug.
    /// * `all` -  Whether to fetch all the images inside the folder.
    /// 
    fn get_from_folder_slug(&self, folder_slug: String, all: bool)
        -> Result<Vec<Image>, DBError>;

    /// Returns images from a project slug.
    /// 
    /// # Arguments
    /// 
    /// * `project_slug` - The project slug.
    /// * `all` -  Whether to fetch all the images inside a project.
    /// 
    fn get_from_project_slug(&self, project_slug: String, all: bool)
        -> Result<Vec::<Image>, DBError>;

    fn add(&self, image: Image) -> Result<u32, String>;

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

    fn update(&self, image: Image) -> Result<String, String>;
    fn remove(&self, id: Image) -> Result<String, String>;
    fn remove_item(&self, id: u32) -> Result<String, String>;
}

pub fn get_image_repository() -> impl ImageRepository {
    let dctxt = get_db_context();

    match dctxt.dbimpl {
        DBImpl::MYSQL => {
            MySQLImageRepository {}
        }
    }
}

