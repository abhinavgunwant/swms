pub mod db;
mod encoding;

use serde::Serialize;
use super::rendition::Rendition;
use super::metadata::Metadata;
use encoding::Encoding;
use crate::repository::item::Item;

#[derive(Serialize)]
pub struct Image {
    name: String, // name vs title in metadata?
    id: u32,
    encoding: Encoding,
    height: u16,
    width: u16,
    metadata_id: u32,
    slug: String
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
        return self.get_renditions();
    }

    fn rendition_for_width(&self, width: u32) -> Rendition {
        return self.get_renditions();
    }

    fn rendition_for_name(&self, name: String) -> Rendition {
        return self.get_renditions();
    }

    fn metadata(&self) -> Metadata {
        return Metadata {
            id: 0,
            image_id: 0,
            title: "Test Image!",
            description: "This is test!",
            description_writer: "Test Author!"
        };
    }
}

impl Item for Image {
    fn get_id(&self) -> u32 {
        return self.id;
    }

    fn get_slug(&self) -> String {
        return self.slug;
    }

    fn get_created_on(&self) -> DateTime {
        return self.createdOn;
    }

    fn get_created_by(&self) -> u16 {
        return self.createdOn;
    }

    fn get_modified_on(&self) -> DateTime {
        return self.createdOn;
    }

    fn get_modified_by(&self) -> u16 {
        return self.createdOn;
    }
}
