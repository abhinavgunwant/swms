mod db;

use db::mysql::MySQLProjectRepository;
use crate::db::{ DBError, DBImpl, get_db_context };
use crate::model::project::Project;

pub trait ProjectRepository {
    fn get(&self, id: u32) -> Result<Project, DBError>;
    fn get_from_slug(&self, slug: String) -> Result<Project, DBError>;
    fn get_all(&self) -> Result<Vec<Project>, DBError>;
    fn get_all_paged(&self, page: u32, page_length: u32) -> Result<Vec<Project>, DBError>;
    fn get_user_projects(&self, user_id: u32) -> Result<Vec::<Project>, DBError>;
    fn add(&self, project: Project);
    fn add_users_to_project(&self, project_id: u32, users: &Vec<u32>);

    /**
     * Validates slug for new project.
     *
     * Checks if any project exists with supplied slug, if not, returns true.
     */
    fn validate_project_slug(&self, slug: String) -> Result<bool, DBError>;
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
