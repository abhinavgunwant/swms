mod db;

use chrono::{ DateTime, Utc };
use crate::db::{ DBError, DBImpl, get_db_context };
use serde::{ Serialize, Deserialize };
use serde_json;
use db::mysql::MySQLProjectRepository;

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

pub trait ProjectRepository {
    fn get(&self, id: u32) -> Result<Project, DBError>;
    fn get_from_slug(&self, slug: String) -> Result<Project, DBError>;
    fn get_all(&self) -> Result<Vec::<Project>, DBError>;
    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<Project>;
    fn get_user_projects(&self, user_id: u32) -> Result<Vec::<Project>, DBError>;
    fn add(&self, project: Project);
    fn add_users_to_project(&self, project_id: u32, users: &Vec<u32>);
    fn update(&self, project: Project);
    fn remove(&self, id: Project);
    fn remove_item(&self, id: u32);
}

pub fn get_project_repository() -> impl ProjectRepository {
    let dctxt = get_db_context();

    match dctxt.dbimpl {
        DBImpl::MYSQL => {
            MySQLProjectRepository {}
        }
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
