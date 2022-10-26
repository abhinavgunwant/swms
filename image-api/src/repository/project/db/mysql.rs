use chrono::Utc;
use mysql::*;
use mysql::prelude::*;
use std::result::Result;
use crate::db::{ DBError, get_db_connection };
use crate::repository::project::{ Project, ProjectRepository };

pub struct MySQLProjectRepository {}

fn get_project_from_row(row_wrapped: Option<&Row>)
    -> std::result::Result<Project, DBError> {

    match row_wrapped {
        Some(row_ref) => {
            let mut row = row_ref.clone();

            Ok(Project {
                id: row.take("ID").unwrap(),
                name: row.take("NAME").unwrap(),
                slug: row.take("SLUG").unwrap(),
                description: row.take("DESCRIPTION").unwrap(),
                restrict_users: row.take("RESTRICT_USERS").unwrap(),
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

impl ProjectRepository for MySQLProjectRepository {
    /**
     * Gets a project based on it's ID.
     */
    fn get(&self, id: u32) -> Result<Project, DBError> {
        let mut conn: PooledConn = get_db_connection();

        let statement = conn.prep(r"
            SELECT ID, NAME, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
            MODIFIED_ON, SLUG, RESTRICT_USERS
            WHERE ID = :id
        ").unwrap();

        let rows: Vec<Row> = conn.exec(statement, params! {"id" => id}).unwrap();
        get_project_from_row(rows.get(0))
    }
    
    /**
     * Gets a project from it's slug.
     */
    fn get_from_slug(&self, slug: String) -> Result<Project, DBError> {
        let mut conn: PooledConn = get_db_connection();

        let statement = conn.prep(r"
            SELECT ID, NAME, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
            MODIFIED_ON, SLUG, RESTRICT_USERS
            WHERE SLUG = :slug
        ").unwrap();

        let rows: Vec<Row> = conn.exec(statement, params! {"slug" => slug}).unwrap();
        get_project_from_row(rows.get(0))
    }

    /**
     * Gets all the projects in the table.
     */
    fn get_all(&self) -> Result<Vec::<Project>, DBError> {
        let mut conn: PooledConn = get_db_connection();

        let projects_wrapped: Result<Vec<Project>, Error> = conn.query_map(
            r"
                SELECT
                ID, NAME, DESCRIPTION, CREATED_BY, MODIFIED_BY, SLUG
                FROM PROJECT
            ",
            |(
                id, name, description, created_by, modified_by, slug,
                // restrict_users
            )| {
                Project {
                    id, name, slug, description, restrict_users: false, created_by,
                    modified_by, created_on: Utc::now(), modified_on: Utc::now(),
                }
            }
        );

        match projects_wrapped {
            Ok (projects) => {
                if projects.len() > 0 {
                    return Ok (projects);
                }

                Err (DBError::NOT_FOUND)
            }

            Err (e) => {
                eprintln!("Error while retrieving all projects: {}", e);
                Err (DBError::OtherError)
            }
        }
    }

    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<Project> {
        let mut conn: PooledConn = get_db_connection();

        let projects: Vec<Project> = conn.query_map(
            format!(r"
                SELECT
                ID, NAME, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG, RESTRICT_USERS
                FROM PROJECT LIMIT {},{}
            ", page*page_length, page),
            |(
                id, name, description, created_by, modified_by, slug,
                restrict_users
            )| {
                Project {
                    id, name, slug, description, restrict_users, created_by,
                    modified_by, created_on: Utc::now(), modified_on: Utc::now(),
                }
            }
        ).unwrap();

        projects
    }

    /**
     * Gets the list of projects that a user has access to.
     */
    fn get_user_projects(&self, user_id: u32) -> Result<Vec::<Project>, DBError> {
        let mut conn: PooledConn = get_db_connection();

        let statement = conn.prep(r"
            SELECT
                P.ID, P.NAME, P.DESCRIPTION, P.CREATED_BY, P.MODIFIED_BY,
                P.SLUG
            FROM PROJECT P, USER_PROJECT UP
            WHERE P.RESTRICT_USERS = FALSE OR (
                P.ID = UP.PROJECT_ID AND UP.USER_ID = :user_id)
        ").unwrap();

        let rows_wrapped: Result<Vec::<Row>, Error> = conn.exec(statement, params! { "user_id" => user_id });

        match rows_wrapped {
            Ok (rows) => {
                let mut projects: Vec<Project> = vec![];

                for row in rows.iter() {
                    let mut r = row.clone();

                    let restrict_users_wrapped = r.take("RESTRICT_USERS");
                    let restrict_users: bool;

                    match restrict_users_wrapped {
                        Some(ru) => {
                            restrict_users = ru;
                        }

                        None => {
                            restrict_users = false;
                        }
                    }

                    projects.push(Project {
                        id: r.take("ID").unwrap(),
                        name: r.take("NAME").unwrap(),
                        slug: r.take("SLUG").unwrap(),
                        description: r.take("DESCRIPTION").unwrap(),
                        restrict_users,
                        created_by: r.take("CREATED_BY").unwrap(),
                        modified_by: 0,
                        created_on: Utc::now(),
                        modified_on: Utc::now(),
                    });
                }

                Ok (projects)
            }

            Err(_e) => {
                Err(DBError::NOT_FOUND)
            }
        }
    }

    fn add(&self, project: Project) {
        let mut conn: PooledConn = get_db_connection();

        conn.exec_drop(r"
            INSERT INTO PROJECT (
                ID, NAME, DESCRIPTION, SLUG, RESTRICT_USERS, CREATED_BY,
                MODIFIED_BY, CREATED_ON, MODIFIED_ON
            ) VALUES (
                :id, :name, :description, :slug, :restrict_users, :created_by,
                NULL, CURRENT_TIMESTAMP(), NULL
            )
        ", params! {
            "id" => &project.id,
            "name" => &project.name,
            "description" => &project.description,
            "slug" => &project.slug,
            "restrict_users" => &project.restrict_users,
            "created_by" => &project.created_by,
        }).expect("Error while creating project");
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

    fn update(&self, project: Project) {
        println!("Updating an project");
    }

    fn remove(&self, id: Project) {
        println!("Updating an project");
    }

    fn remove_item(&self, id: u32) {
        println!("Updating an project");
    }
}
