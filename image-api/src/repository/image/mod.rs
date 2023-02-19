pub mod db;

use serde_json;

use crate::db::{ DBImpl, get_db_context, DBError };
use crate::model::{ encoding::Encoding, image::Image };
use db::mysql::MySQLImageRepository;

pub trait ImageRepository {
    fn get(&self, id: u32) -> Result<Image, DBError>;
    fn get_all(&self) -> Result<Vec<Image>, DBError>;
    fn get_all_from_project(&self, project_id: u32)
        -> Result<Vec::<Image>, DBError>;
    fn get_all_from_project_slug(&self, project_slug: String)
        -> Result<Vec::<Image>, DBError>;
    fn get_all_paged(&self, page: u32, page_length: u32) -> Result<Vec<Image>, DBError>;
    fn add(&self, image: Image) -> Result<u32, String>;
    fn update(&self, image: Image) -> Result<String, String>;
    fn remove(&self, id: Image);
    fn remove_item(&self, id: u32);
}

pub fn get_image_repository() -> impl ImageRepository {
    let dctxt = get_db_context();

    match dctxt.dbimpl {
        DBImpl::MYSQL => {
            MySQLImageRepository {}
        }
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Image {}", serde_json::to_string(&self).unwrap())
    }
}
