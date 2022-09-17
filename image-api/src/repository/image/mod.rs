pub mod db;
pub mod encoding;

use serde::Serialize;
use super::rendition::Rendition;
use super::metadata::Metadata;
use encoding::Encoding;
use chrono::{DateTime, Utc};
use crate::repository::item::Item;
use crate::db::{ CURRENT_DB, DBImpl, get_db_context, dbcontext::{ DBContext } };
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
    pub metadata_id: u32,
    pub slug: String,
    pub created_on: DateTime<Utc>,
    pub created_by: u16,
    pub modified_on: DateTime<Utc>,
    pub modified_by: u16
}

pub trait ImageRepository {
    fn get(&self, id: u32) -> Image;
    fn get_all(&self) -> Vec::<Image>;
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


// TODO: Decide on whether to use this or not.
//pub trait ImageItem : Item {
//    fn get(&self) -> Image;
//    fn get_all(&self) -> Vec<Rendition>;
//    fn renditions_for_device(&self, device: String) -> Vec<Rendition>;
//    fn rendition_for_width(&self, width: u32) -> Rendition;
//    fn rendition_for_name(&self, name: String) -> Rendition;
//    fn metadata(&self) -> Metadata;
//}
//
//impl ImageItem for Image {
//    fn get(&self) -> Image {
//        println!("curent db: {}", CURRENT_DB.db_name());
//
//        Image {
//            name: "test".to_string(),
//            id: 0,
//            encoding: Encoding::JPG,
//            height: 0,
//            width: 0,
//            metadata_id: 0,
//            slug: "test".to_string(),
//            created_on: Utc::now(),
//            created_by: 0,
//            modified_on: Utc::now(),
//            modified_by: 0
//        }
//    }
//
//    fn get_all(&self) -> Vec<Rendition> {
//        let mut renditions = Vec::new();
//
//        renditions.push(Rendition {
//            id: 0,
//            image_id: 0,
//            width: 0,
//            height: 0,
//            slug: "Rendition 1".to_string()
//        });
//        renditions.push(Rendition {
//            id: 1,
//            image_id: 0,
//            width: 0,
//            height: 0,
//            slug: "Rendition 2".to_string()
//        });
//        renditions.push(Rendition {
//            id: 2,
//            image_id: 0,
//            width: 0,
//            height: 0,
//            slug: "Rendition 3".to_string()
//        });
//        renditions.push(Rendition {
//            id: 3,
//            image_id: 0,
//            width: 0,
//            height: 0,
//            slug: "Rendition 4".to_string()
//        });
//
//        return renditions;
//    }
//
//    fn renditions_for_device(&self, device: String) -> Vec<Rendition> {
//        return self.get_all();
//    }
//
//    fn rendition_for_width(&self, width: u32) -> Rendition {
//        return Rendition {
//            id: 3,
//            image_id: 0,
//            width: 0,
//            height: 0,
//            slug: "Rendition 4".to_string()
//        };
//    }
//
//    fn rendition_for_name(&self, name: String) -> Rendition {
//        return Rendition {
//            id: 3,
//            image_id: 0,
//            width: 0,
//            height: 0,
//            slug: "Rendition 4".to_string()
//        };
//    }
//
//    fn metadata(&self) -> Metadata {
//        return Metadata {
//            id: 0,
//            image_id: 0,
//            title: "Test Image!".to_string(),
//            description: "This is test!".to_string(),
//            description_writer: "Test Author!".to_string()
//        };
//    }
//}
//
//impl Item for Image {
//    fn id(&self) -> u32 {
//        return self.id;
//    }
//
//    fn slug(&self) -> String {
//        self.slug.clone()
//    }
//
//    fn created_on(&self) -> DateTime<Utc> {
//        return self.created_on;
//    }
//
//    fn created_by(&self) -> u16 {
//        return self.created_by;
//    }
//
//    fn modified_on(&self) -> DateTime<Utc> {
//        return self.modified_on;
//    }
//
//    fn modified_by(&self) -> u16 {
//        return self.modified_by;
//    }
//}

//impl <T: ?Sized> ImageItem for Box<T> where T: ImageItem {
//    fn get_all(&self) -> Vec<Rendition> {
//        (**self).get_all()
//    }
//
//    fn renditions_for_device(&self, device: String) -> Vec<Rendition> {
//        (**self).renditions_for_device(device)
//    }
//
//    fn rendition_for_width(&self, width: u32) -> Rendition {
//        (**self).rendition_for_width(width)
//    }
//
//    fn rendition_for_name(&self, name: String) -> Rendition {
//        (**self).rendition_for_name(name)
//    }
//
//    fn metadata(&self) -> Metadata {
//        (**self).metadata()
//    }
//}

