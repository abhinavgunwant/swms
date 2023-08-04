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
     * Validates if a project with the provided slug doesn't exists.
     *
     * Used for providing real-time validation while the admin is typing the
     * project name (or project slug) in "New Project" screen.
     *
     * `slug`: The slug provided (should be `lowercase`).
     *
     * Returns true if a project with the supplied slug doesn't exist.
     */
    fn validate_new_project_slug(&self, slug: String) -> Result<bool, DBError>;

    /**
     * Validates if a project with the provided slug exists.
     *
     * Behaves exactly opposite to `validate_new_project_slug`.
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
