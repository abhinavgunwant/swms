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
