pub mod db;
mod encoding;

use serde::Serialize;
use super::rendition::Rendition;
use super::metadata::Metadata;
use encoding::Encoding;
use chrono::{DateTime, Utc};
use crate::repository::item::Item;

#[derive(Serialize)]
pub struct Image {
    name: String, // name vs title in metadata?
    id: u32,
    encoding: Encoding,
    height: u16,
    width: u16,
    metadata_id: u32,
    slug: String,
    created_on: DateTime<Utc>,
    created_by: u16,
    modified_on: DateTime<Utc>,
    modified_by: u16
}

trait ImageItem : Item {
    fn get_all(&self) -> Vec<Rendition>;
    fn renditions_for_device(&self, device: String) -> Vec<Rendition>;
    fn rendition_for_width(&self, width: u32) -> Rendition;
    fn rendition_for_name(&self, name: String) -> Rendition;
    fn metadata(&self) -> Metadata;
}

impl ImageItem for Image {
    fn get_all(&self) -> Vec<Rendition> {
        let mut renditions = Vec::new();

        renditions.push(Rendition {
            id: 0,
            image_id: 0,
            width: 0,
            height: 0,
            slug: "Rendition 1".to_string()
        });
        renditions.push(Rendition {
            id: 1,
            image_id: 0,
            width: 0,
            height: 0,
            slug: "Rendition 2".to_string()
        });
        renditions.push(Rendition {
            id: 2,
            image_id: 0,
            width: 0,
            height: 0,
            slug: "Rendition 3".to_string()
        });
        renditions.push(Rendition {
            id: 3,
            image_id: 0,
            width: 0,
            height: 0,
            slug: "Rendition 4".to_string()
        });

        return renditions;
    }

    fn renditions_for_device(&self, device: String) -> Vec<Rendition> {
        return self.get_all();
    }

    fn rendition_for_width(&self, width: u32) -> Rendition {
        return Rendition {
            id: 3,
            image_id: 0,
            width: 0,
            height: 0,
            slug: "Rendition 4".to_string()
        };
    }

    fn rendition_for_name(&self, name: String) -> Rendition {
        return Rendition {
            id: 3,
            image_id: 0,
            width: 0,
            height: 0,
            slug: "Rendition 4".to_string()
        };
    }

    fn metadata(&self) -> Metadata {
        return Metadata {
            id: 0,
            image_id: 0,
            title: "Test Image!".to_string(),
            description: "This is test!".to_string(),
            description_writer: "Test Author!".to_string()
        };
    }
}

impl Item for Image {
    fn id(&self) -> u32 {
        return self.id;
    }

    fn slug(&self) -> String {
        return self.slug;
    }

    fn created_on(&self) -> DateTime<Utc> {
        return self.created_on;
    }

    fn created_by(&self) -> u16 {
        return self.created_by;
    }

    fn modified_on(&self) -> DateTime<Utc> {
        return self.modified_on;
    }

    fn modified_by(&self) -> u16 {
        return self.modified_by;
    }
}
