use chrono::Utc;
use mysql::*;
use mysql::prelude::*;
use std::result::Result;
use crate::db::{
    utils::mysql::{ get_row_from_query, get_rows_from_query },
    DBError, get_db_connection,
};
use crate::repository::project::{ Project, ProjectRepository };

pub struct MySQLProjectRepository {}

fn get_project_from_row(row_wrapped: Result<Option<Row>, Error>)
    -> std::result::Result<Project, DBError> {
    match row_wrapped {
        Ok(row_option) => {
            match row_option {
                Some(row_ref) => {
                    let mut row: Row = row_ref.clone();
                    let restrict_users: bool;

                    match row.take("RESTRICT_USERS") {
                        Some(ru) => { restrict_users = ru; }
                        None => { restrict_users = false; }
                    }

                    Ok(Project {
                        id: row.take("ID").unwrap(),
                        name: row.take("NAME").unwrap(),
                        slug: row.take("SLUG").unwrap(),
                        description: row.take("DESCRIPTION").unwrap(),
                        restrict_users,
                        created_by: row.take("CREATED_BY").unwrap(),
                        modified_by: row.take("MODIFIED_BY").unwrap(),
                        created_on: Utc::now(),
                        modified_on: Utc::now(),
                    })
                }

                None => {
                    Err(DBError::NOT_FOUND)
                }
            }
        }

        Err (e) => {
            eprintln!("Error while getting rendition from query: {}", e);

            Err(DBError::OtherError)
        }
    }
}

fn get_projects_from_row(row_wrapped: Result<Vec<Row>, Error>)
    -> std::result::Result<Vec<Project>, DBError> {
    match row_wrapped {
        Ok (rows) => {
            let mut projects: Vec<Project> = vec![];

            for row_ in rows.iter() {
                let mut row = row_.clone();

                let restrict_users: bool;
                let modified_by: u32;

                match row.take("RESTRICT_USERS") {
                    Some(ru) => { restrict_users = ru; }
                    None => { restrict_users = false; }
                }

                match row.take_opt("MODIFIED_BY") {
                    Some(mb_result) => {
                        match mb_result {
                            Ok (mb) => { modified_by = mb; }
                            Err (_e) => { modified_by = 0; }
                        }
                    }
                    None => { modified_by = 0; }
                }

                projects.push(Project {
                    id: row.take("ID").unwrap(),
                    name: row.take("NAME").unwrap(),
                    slug: row.take("SLUG").unwrap(),
                    description: row.take("DESCRIPTION").unwrap(),
                    restrict_users,
                    created_by: row.take("CREATED_BY").unwrap(),
                    modified_by,
                    created_on: Utc::now(),
                    modified_on: Utc::now(),
                });
            }

            Ok (projects)
        }

        Err(e) => {
            eprintln!("Error while getting images from query: {}", e);

            Err(DBError::NOT_FOUND)
        }
    }
}

impl ProjectRepository for MySQLProjectRepository {
    /**
     * Gets a project based on it's ID.
     */
    fn get(&self, id: u32) -> Result<Project, DBError> {
        get_project_from_row(get_row_from_query(
            r"SELECT
                ID, NAME, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG, RESTRICT_USERS WHERE ID = :id",
            params! { "id" => id },
        ))
    }
    
    /**
     * Gets a project from it's slug.
     */
    fn get_from_slug(&self, slug: String) -> Result<Project, DBError> {
        get_project_from_row(get_row_from_query(
            r"SELECT
                ID, NAME, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG, RESTRICT_USERS WHERE SLUG = :slug",
            params! {"slug" => slug},
        ))
    }

    /**
     * Gets all the projects in the table.
     */
    fn get_all(&self) -> Result<Vec::<Project>, DBError> {
        get_projects_from_row(get_rows_from_query(
            r"SELECT
                ID, NAME, DESCRIPTION, CREATED_BY, MODIFIED_BY, SLUG
            FROM PROJECT",
            Params::Empty
        ))
    }

    fn get_all_paged(&self, page: u32, page_length: u32) -> Result<Vec<Project>, DBError> {
        get_projects_from_row(get_rows_from_query(
            r"SELECT
                ID, NAME, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG, RESTRICT_USERS
                FROM PROJECT LIMIT :page1, :page2",
            params! { "page1" => page*page_length, "page2" => page }
        ))
    }

    /**
     * Gets the list of projects that a user has access to.
     */
    fn get_user_projects(&self, user_id: u32) -> Result<Vec::<Project>, DBError> {
        get_projects_from_row(get_rows_from_query(
            r"SELECT
                P.ID, P.NAME, P.DESCRIPTION, P.CREATED_BY, P.MODIFIED_BY,
                P.SLUG
            FROM PROJECT P WHERE P.RESTRICT_USERS = FALSE
            UNION
            SELECT
                P.ID, P.NAME, P.DESCRIPTION, P.CREATED_BY, P.MODIFIED_BY,
                P.SLUG
            FROM PROJECT P, USER_PROJECT UP
            WHERE P.RESTRICT_USERS = TRUE AND P.ID = UP.PROJECT_ID
                AND UP.USER_ID = :user_id",
            params! { "user_id" => user_id }
        ))
    }

    fn add(&self, project: Project) {
        let mut conn: PooledConn = get_db_connection();

        conn.exec_drop(
            r"INSERT INTO PROJECT (
                ID, NAME, DESCRIPTION, SLUG, RESTRICT_USERS, CREATED_BY,
                MODIFIED_BY, CREATED_ON, MODIFIED_ON
            ) VALUES (
                :id, :name, :description, :slug, :restrict_users, :created_by,
                NULL, CURRENT_TIMESTAMP(), NULL
            )",
            params! {
                "id" => &project.id,
                "name" => &project.name,
                "description" => &project.description,
                "slug" => &project.slug,
                "restrict_users" => &project.restrict_users,
                "created_by" => &project.created_by,
            }
        ).expect("Error while creating project");
    }
    
    /**
     * Adds a list of users to a project.
     * Takes in a u32 vector containing user IDs.
     */
    fn add_users_to_project(&self, project_id: u32, users: &Vec<u32>) {
        let mut conn: PooledConn = get_db_connection();
        
        // TODO: Remove the users who already have access to the project
        // `project_id` here...

        conn.exec_batch(r"
            INSERT INTO USER_PROJECT ( USER_ID, PROJECT_ID )
            VALUES ( :user_id, :project_id )
        ", users.iter().map(|user| params! {
            "project_id" => &project_id,
            "user_id" => user,
        })).expect("Error while adding users to project");
    }

    fn is_valid_new_slug(&self, slug: String) -> Result<bool, DBError> {
        let row_result: Result<Option<Row>,Error> = get_row_from_query(
            r"SELECT NOT EXISTS (
                SELECT ID FROM PROJECT WHERE SLUG = :slug
            ) AS VALID",
            params! { "slug" => slug }
        );

        match row_result {
            Ok (row_option) => {
                match row_option {
                    Some (r) => {
                        let mut row = r;

                        let valid: bool = row.take("VALID").unwrap();

                        Ok (valid)
                    }

                    None => {
                        Ok (true)
                    }
                }
            }

            Err (_e) => {
                Err (DBError::OtherError)
            }
        }
    }

    fn is_valid_slug(&self, slug: String) -> Result<Option<u32>, DBError> {
        let row_result: Result<Option<Row>,Error> = get_row_from_query(
            r"SELECT ID FROM PROJECT WHERE SLUG = :slug",
            params! { "slug" => slug }
        );

        match row_result {
            Ok (row_option) => {
                match row_option {
                    Some (r) => {
                        let mut row = r;

                        match row.take("ID") {
                            Some (id) => Ok(Some(id)),
                            None => Ok(None),
                        }
                    }

                    None => Ok(None),
                }
            }

            Err (_e) => {
                Err (DBError::OtherError)
            }
        }
    }

    fn update(&self, _project: Project) {
        // TODO: Implement
        println!("Updating a project");
    }

    fn remove(&self, _id: Project) {
        // TODO: Implement
        println!("Updating a project");
    }

    fn remove_item(&self, _id: u32) {
        // TODO: Implement
        println!("Updating a project");
    }
}

