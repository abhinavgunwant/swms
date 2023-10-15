pub mod db;

use crate::{
    server::db::DBError, model::{ encoding::Encoding, rendition::Rendition }
};

pub trait RenditionRepository {
    fn get(&mut self, id: u32) -> Result<Rendition, DBError>;
    fn get_from_project_rendition_slug(&mut self, p_slug: String, i_slug: String)
        -> Result<Rendition, DBError>;
    fn get_from_folder_rendition_slug(&mut self, f_slug: String, i_slug: String)
        -> Result<Rendition, DBError>;
    fn get_from_image_and_slug(&mut self, image_id: u32, slug: String) -> Result<Rendition, DBError>;
    fn get_all(&mut self) -> Result<Vec<Rendition>, DBError>;
    fn get_all_from_image(&mut self, image_id: u32) -> Result<Vec::<Rendition>, DBError>;
    fn get_all_from_project(&mut self, project_id: u32)
        -> Result<Vec::<Rendition>, DBError>;
    fn get_all_from_project_slug(&mut self, project_slug: String)
        -> Result<Vec::<Rendition>, DBError>;
    fn get_all_paged(&mut self, page: u32, page_length: u32) -> Result<Vec<Rendition>, DBError>;
    fn add(&mut self, rendition: Rendition) -> Result<u32, String>;

    /// Validates if a rendition with the provided slug doesn't exists for a
    /// given image.
    /// 
    /// Used for providing real-time validation while the admin is typing the
    /// project name (or project slug) in "New Project" screen.
    /// 
    /// `slug`: The slug provided (should be `lowercase`).
    /// 
    /// Returns true if a project with the supplied slug doesn't exist.
    fn is_valid_new_slug(&mut self, image_id: u32, slug: String) -> Result<bool, DBError>;

    /// Validates if a rendition with the provided slug exists for image.
    /// 
    /// Behaves exactly opposite to `validate_new_project_slug`.
    fn is_valid_slug(&mut self, image_id: u32, slug: String) -> Result<bool, DBError>;

    fn update(&mut self, rendition: Rendition);
    fn remove(&mut self, rendition: Rendition) -> Result<String, String>;
    fn remove_item(&mut self, id: u32) -> Result<String, String>;
    fn remove_all_from_image(&mut self, image_id: u32) -> Result<String, String>;
}

