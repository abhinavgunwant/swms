use serde::{ Serialize, Deserialize};
use serde_json;
use chrono::{ DateTime, Utc };
use crate::model::encoding::Encoding;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: u32,
    pub name: String, // Original Filename
    pub title: String,
    pub slug: String,
    pub encoding: Encoding,
    pub height: u16,
    pub width: u16,
    pub is_published: bool,
    pub project_id: u32,
    pub folder_id: u32,
    pub created_on: DateTime<Utc>,
    pub created_by: u16,
    pub modified_on: DateTime<Utc>,
    pub modified_by: u16,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Image {}", serde_json::to_string(&self).unwrap())
    }
}

