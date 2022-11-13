use serde::Serialize;
use chrono::{ DateTime, Utc };
use crate::model::encoding::Encoding;

#[derive(Serialize, Clone)]
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
    pub created_on: DateTime<Utc>,
    pub created_by: u16,
    pub modified_on: DateTime<Utc>,
    pub modified_by: u16,
}
