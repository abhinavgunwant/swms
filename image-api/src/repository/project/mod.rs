pub mod db;

use crate::{ server::db::DBError, model::project::Project };

pub trait ProjectRepository {
    fn get(&mut self, id: u32) -> Result<Project, DBError>;
    fn get_from_slug(&mut self, slug: String) -> Result<Project, DBError>;
    fn get_all(&mut self) -> Result<Vec<Project>, DBError>;
    fn get_all_paged(&mut self, page: u32, page_length: u32) -> Result<Vec<Project>, DBError>;
    fn get_user_projects(&mut self, user_id: u32) -> Result<Vec::<Project>, DBError>;
    fn add(&mut self, project: Project);
    fn add_users_to_project(&mut self, project_id: u32, users: &Vec<u32>);

    
    /// Validates if a project with the provided slug doesn't exists.
    /// 
    /// Used for providing real-time validation while the admin is typing the
    /// project name (or project slug) in "New Project" screen.
    /// 
    /// `slug`: The slug provided (should be `lowercase`).
    /// 
    /// Returns true if a project with the supplied slug doesn't exist.
    fn is_valid_new_slug(&mut self, slug: String) -> Result<bool, DBError>;

    /// Validates if a project with the provided slug exists.
    /// 
    /// Behaves exactly opposite to `validate_new_project_slug`.
    fn is_valid_slug(&mut self, slug: String) -> Result<Option<u32>, DBError>;

    fn update(&mut self, project: Project);
    fn remove(&mut self, id: Project) -> Result<String, String>;
    fn remove_item(&mut self, id: u16) -> Result<String, String>;
    fn remove_multiple(&mut self, id: Vec<u16>) -> Result<String, String>;
}

