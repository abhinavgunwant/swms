pub mod db;
pub mod encoding;

use serde::Serialize;
use encoding::Encoding;
use chrono::{DateTime, Utc};
use crate::db::{ DBImpl, get_db_context, DBError };
use db::mysql::MySQLImageRepository;

#[derive(Serialize)]
pub struct Image {
    pub id: u32,
    pub name: String, // Original Filename
    pub title: String,
    pub encoding: Encoding,
    pub height: u16,
    pub width: u16,
    pub is_published: bool,
    pub project_id: u32,
    pub folder_id: u32,
    // pub metadata_id: u32,
    pub slug: String,
    pub created_on: DateTime<Utc>,
    pub created_by: u16,
    pub modified_on: DateTime<Utc>,
    pub modified_by: u16
}

pub trait ImageRepository {
    fn get(&self, id: u32) -> Image;
    fn get_all(&self) -> Vec::<Image>;
    fn get_all_from_project(&self, project_id: u32)
        -> Result<Vec::<Image>, DBError>;
    fn get_all_from_project_slug(&self, project_slug: String)
        -> Result<Vec::<Image>, DBError>;
    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<Image>;
    fn add(&self, image: Image);
    fn update(&self, image: Image);
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
