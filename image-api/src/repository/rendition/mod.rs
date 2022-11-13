pub mod db;
pub mod encoding;

use serde::Serialize;
use serde_json;
use encoding::Encoding;
use chrono::{ DateTime, Utc };
use crate::db::{ DBImpl, get_db_context, DBError };
use db::mysql::MySQLRenditionRepository;

#[derive(Serialize, Clone)]
pub struct Rendition {
    pub id: u32,
    pub image_id: u32,
    pub height: u16,
    pub width: u16,
    pub target_device: String,
    pub slug: String,
    pub is_published: bool,
    pub encoding: Encoding,
    pub created_on: DateTime<Utc>,
    pub created_by: u16,
    pub modified_on: DateTime<Utc>,
    pub modified_by: u16
}

pub trait RenditionRepository {
    fn get(&self, id: u32) -> Result<Rendition, DBError>;
    fn get_from_project_rendition_slug(&self, p_slug: String, i_slug: String)
        -> Result<Rendition, DBError>;
    fn get_from_folder_rendition_slug(&self, f_slug: String, i_slug: String)
        -> Result<Rendition, DBError>;
    fn get_all(&self) -> Vec::<Rendition>;
    fn get_all_from_image(&self, image_id: u32) -> Result<Vec::<Rendition>, DBError>;
    fn get_all_from_project(&self, project_id: u32)
        -> Result<Vec::<Rendition>, DBError>;
    fn get_all_from_project_slug(&self, project_slug: String)
        -> Result<Vec::<Rendition>, DBError>;
    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<Rendition>;
    fn add(&self, rendition: Rendition);
    fn update(&self, rendition: Rendition);
    fn remove(&self, id: Rendition);
    fn remove_item(&self, id: u32);
}

pub fn get_rendition_repository() -> impl RenditionRepository {
    let dctxt = get_db_context();

    match dctxt.dbimpl {
        DBImpl::MYSQL => {
            MySQLRenditionRepository {}
        }
    }
}

impl std::fmt::Display for Rendition  {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Image {}", serde_json::to_string(&self).unwrap())
    }
}

impl std::default::Default for Rendition {
    fn default() -> Self {
        Rendition {
            id: 0, 
            image_id: 0,
            height: 0,
            width: 0,
            target_device: String::from(""),
            slug: String::from(""),
            is_published: true,
            encoding: Encoding::JPG,
            created_on: Utc::now(),
            created_by: 0,
            modified_on: Utc::now(),
            modified_by: 0,
        }
    }
}
