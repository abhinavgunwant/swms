use serde::Serialize;
use serde_json;
use chrono::{ DateTime, Utc };

use crate::model::encoding::Encoding;

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
