pub mod db;

use crate::{
    db::{ DBImpl, get_db_context, DBError },
    model::{ encoding::Encoding, rendition::Rendition },
};
use db::mysql::MySQLRenditionRepository;

pub trait RenditionRepository {
    fn get(&self, id: u32) -> Result<Rendition, DBError>;
    fn get_from_project_rendition_slug(&self, p_slug: String, i_slug: String)
        -> Result<Rendition, DBError>;
    fn get_from_folder_rendition_slug(&self, f_slug: String, i_slug: String)
        -> Result<Rendition, DBError>;
    fn get_all(&self) -> Result<Vec<Rendition>, DBError>;
    fn get_all_from_image(&self, image_id: u32) -> Result<Vec::<Rendition>, DBError>;
    fn get_all_from_project(&self, project_id: u32)
        -> Result<Vec::<Rendition>, DBError>;
    fn get_all_from_project_slug(&self, project_slug: String)
        -> Result<Vec::<Rendition>, DBError>;
    fn get_all_paged(&self, page: u32, page_length: u32) -> Result<Vec<Rendition>, DBError>;
    fn add(&self, rendition: Rendition) -> Result<u32, String>;

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
    fn is_valid_new_slug(&self, image_id: u32, slug: String) -> Result<bool, DBError>;

    /**
     * Validates if a rendition with the provided slug exists for image.
     *
     * Behaves exactly opposite to `validate_new_project_slug`.
     */
    fn is_valid_slug(&self, image_id: u32, slug: String) -> Result<bool, DBError>;

    fn update(&self, rendition: Rendition);
    fn remove(&self, rendition: Rendition) -> Result<String, String>;
    fn remove_item(&self, id: u32) -> Result<String, String>;
    fn remove_all_from_image(&self, image_id: u32) -> Result<String, String>;
}

pub fn get_rendition_repository() -> impl RenditionRepository {
    let dctxt = get_db_context();

    match dctxt.dbimpl {
        DBImpl::MYSQL => {
            MySQLRenditionRepository {}
        }
    }
}
