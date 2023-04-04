use serde::{ Serialize, Deserialize };
use serde_json;
use chrono::{ DateTime, Utc };

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

impl std::fmt::Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Folder {}", serde_json::to_string(&self).unwrap())
    }
}

