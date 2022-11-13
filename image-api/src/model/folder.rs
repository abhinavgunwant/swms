use serde::{ Serialize, Deserialize };
use chrono::{ DateTime, Utc };

#[derive(Serialize, Deserialize)]
pub struct Folder {
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub project_id: u16,
    pub description: String,
    pub parent_folder_id: u32,
    pub created_by: u32,
    pub modified_by: u32,
    pub created_on: DateTime<Utc>,
    pub modified_on: DateTime<Utc>
}
