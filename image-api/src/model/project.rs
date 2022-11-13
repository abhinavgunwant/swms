use chrono::{ DateTime, Utc };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(default = "Project::default")]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: u16,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub restrict_users: bool,
    pub created_by: u32,
    pub modified_by: u32,
    pub created_on: DateTime<Utc>,
    pub modified_on: DateTime<Utc>
}

impl Default for Project {
    fn default() -> Project {
        Project {
            id: 0,
            name: String::from(""),
            slug: String::from(""),
            description: String::from(""),
            restrict_users: true,
            created_by: 0,
            modified_by: 0,
            created_on: Utc::now(),
            modified_on: Utc::now(),
        }
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Project {}", serde_json::to_string(&self).unwrap())
    }
}

pub fn validate_project(project: &Project) -> (bool, Vec<String>) {
    let mut valid: bool = true;
    let mut error_msgs: Vec<String> = vec![];

    if project.name.eq("") {
        valid = false;
        error_msgs.push(String::from("Name cannot be empty"));
    }

    if project.slug.eq("") {
        valid = false;
        error_msgs.push(String::from("Slug cannot be empty"));
    }

    (valid, error_msgs)
}
