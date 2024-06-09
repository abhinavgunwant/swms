pub mod image;
pub mod user;
pub mod item;
pub mod rendition;
pub mod project;
pub mod folder;
pub mod role;

use std::sync::Arc;
use mysql::Pool;
use log::{ info, error, debug };

use crate::server::{
    db::{ DBError, mysql_to_db_error }, config::{ ServerConfig, DBType } };

use self::{
    folder::{ FolderRepository, db::mysql::MySQLFolderRepository },
    role::{ RoleRepository, db::mysql::MySQLRoleRepository },
    image::{ ImageRepository, db::mysql::MySQLImageRepository },
    project::{ ProjectRepository, db::mysql::MySQLProjectRepository },
    rendition::{ RenditionRepository, db::mysql::MySQLRenditionRepository },
    user::{ UserRepository, db::mysql::MySQLUserRepository },
};

pub trait Repository {
    /// Does a startup check of the database:
    /// 1. Connection to the db server works (if applicable).
    /// 2. All tables exist in the required database.
    fn startup_check(&self) -> RepositoryStartupStatus;

    /// Initializes the repository:
    /// 1. Creates database.
    /// 2. Creates tables.
    /// 3. Enters some default data to the tables.
    fn init_repo(&self) -> Result<(), RepositoryStartupError>;
    fn get_role_repo(&self) -> Result<Box::<dyn RoleRepository>, DBError>;
    fn get_folder_repo(&self) -> Result<Box::<dyn FolderRepository>, DBError>;
    fn get_image_repo(&self) -> Result<Box::<dyn ImageRepository>, DBError>;
    fn get_project_repo(&self) -> Result<Box::<dyn ProjectRepository>, DBError>;
    fn get_rendition_repo(&self) -> Result<Box::<dyn RenditionRepository>, DBError>;
    fn get_user_repo(&self) -> Result<Box::<dyn UserRepository>, DBError>;
}

const MYSQL_TABLE_LIST: &'static[&'static str] = &[
    "FOLDER", "IMAGE", "PROJECT", "IMAGE_RENDITION", "USER_ROLE", "USER"
];

pub enum RepositoryStartupError {
    ConnectionError,
    DatabaseDoesNotExist,

    /// List of tables relevant to SWMS that were not found.
    TablesMissing(Vec<String>),

    OtherError(String),

    // TODO: Future! Name of tables (key) and the columns that do not exist
    // ColumnsDoNotExist(HashMap<String, Vec<String>>);
}

/// Represents whether the repository exists and is configured correctly.
/// Returns errors if not.
pub enum RepositoryStartupStatus {
    /// Everything is okay
    OK,
    Error(RepositoryStartupError),
}

#[derive(Clone)]
pub struct MySQLRepository {
    connection_pool: Pool,
}

impl Repository for MySQLRepository {
    fn startup_check(&self) -> RepositoryStartupStatus {
        let mut absent_tables: Vec<String> = vec![];

        // Folder repo
        match self.get_folder_repo() {
            Ok(mut folder_repo) => {
                match folder_repo.verify() {
                    Ok(()) => {}

                    Err(db_error) => {
                        match db_error {
                            DBError::NotFound => {
                                debug!("Folder repository not found. Adding it to the absent table list.");

                                absent_tables.push(
                                    String::from(MYSQL_TABLE_LIST[0])
                                );
                            }

                            _ => {
                                error!(
                                    "Some unknown error occured while verifying folder repository! {}",
                                    db_error
                                );
                            }
                        }
                    }
                }
            }

            Err(db_error) => {
                error!(
                    "Some unknown error occured while verifying folder repository! {}",
                    db_error
                );
            }
        }

        if !absent_tables.is_empty() {
            return RepositoryStartupStatus::Error(
                RepositoryStartupError::TablesMissing(absent_tables)
            );
        }

        return RepositoryStartupStatus::OK;
    }

    fn init_repo(&self) -> Result<(), RepositoryStartupError> {
        return Ok(()); 
    }

    fn get_role_repo(&self) -> Result<Box::<dyn RoleRepository>, DBError> {
        match self.connection_pool.get_conn() {
            Ok(connection) => Ok(Box::new(MySQLRoleRepository { connection })),
            Err(e) => Err(
                mysql_to_db_error("Error while creating connection", e)
            ),
        }
    }

    fn get_folder_repo(&self) -> Result<Box::<dyn FolderRepository>, DBError> {
        match self.connection_pool.get_conn() {
            Ok(connection) => Ok(Box::new(MySQLFolderRepository { connection })),
            Err(e) => Err(
                mysql_to_db_error("Error while creating connection", e)
            ),
        }
    }

    fn get_image_repo(&self) -> Result<Box::<dyn ImageRepository>, DBError> {
        match self.connection_pool.get_conn() {
            Ok(connection) => Ok(Box::new(MySQLImageRepository { connection })),
            Err(e) => Err(
                mysql_to_db_error("Error while creating connection", e)
            ),
        }
    }

    fn get_project_repo(&self) -> Result<Box::<dyn ProjectRepository>, DBError> {
        match self.connection_pool.get_conn() {
            Ok(connection) => Ok(Box::new(MySQLProjectRepository { connection })),
            Err(e) => Err(
                mysql_to_db_error("Error while creating connection", e)
            ),
        }
    }

    fn get_rendition_repo(&self) -> Result<Box::<dyn RenditionRepository>, DBError> {
        match self.connection_pool.get_conn() {
            Ok(connection) => Ok(Box::new(MySQLRenditionRepository { connection })),
            Err(e) => Err(
                mysql_to_db_error("Error while creating connection", e)
            ),
        }
    }

    fn get_user_repo(&self) -> Result<Box::<dyn UserRepository>, DBError> {
        match self.connection_pool.get_conn() {
            Ok(connection) => Ok(Box::new(MySQLUserRepository { connection })),
            Err(e) => Err(
                mysql_to_db_error("Error while creating connection", e)
            ),
        }
    }
}

impl MySQLRepository {
    pub fn new(connection_str: &str) -> Result<Self, DBError> {
        match Pool::new(connection_str) {
            Ok(pool) => Ok(MySQLRepository { connection_pool: pool }),
            Err(e) => {
                match e {
                    mysql::Error::MySqlError(ref err) => {
                        if err.code == 1045 || err.code == 1049 {
                            error!(
                                "MySQL Error: {} SQLSTATE: {}. [[ SWMS Hint: Please check your MySQL connection details on the \"config.yml\" file. For locating your config.yml file, check the logs above. ]]",
                                err.message,
                                err.state,
                            );

                            match err.code {
                                1045 => return Err(DBError::AccessDenied),
                                1049 => return Err(DBError::NotFound),

                                _ => return Err(DBError::OtherError),
                            }
                            
                        }

                        Err(mysql_to_db_error(
                            "Error while creating connection pool for MySQL DB", e
                        ))
                    }

                    _ => {
                        Err(mysql_to_db_error(
                            "Error while creating connection pool for MySQL DB", e
                        ))
                    }
                }
            },
        }
    }
}

pub fn create_table(repo: MySQLRepository, table_key: &str) {
    match table_key {
        "FOLDER" => {
            todo!("This will create table!");
        }

        _ => {}
    }
}

/// Gets the repository based on the server configuration.
///
/// When errors are encountered, it prints the error and returns `None`.
/// **Note:** The main function exits when it encounters the `None` return.
pub fn get_repository() -> Result<
        Arc<dyn Repository + Sync + Send>,
        RepositoryStartupError
    > {
    let server_config = ServerConfig::default();

    let connection_string = server_config.get_connection_string();
    let conn_str = connection_string.as_str();

    match server_config.db.db_type {
        DBType::MySQL => {
            match MySQLRepository::new(conn_str) {
                Ok(r) => {
                    info!("MySQL repository startup check in progress!");

                    match r.startup_check() {
                        RepositoryStartupStatus::OK => {
                            info!("MySQL repository startup check successful!");
                        }

                        RepositoryStartupStatus::Error(repo_startup_error) => {
                            match repo_startup_error {
                                RepositoryStartupError::DatabaseDoesNotExist => {
                                    info!("Connected to MySQL server.");
                                    info!("The required database was not found. Creating new...");

                                    let _ = r.init_repo();
                                }

                                RepositoryStartupError::TablesMissing(tables) => {
                                    info!("Connected to MySQL server.");
                                    error!("Some tables do not exist, trying to create them...");

                                    for mysql_table in MYSQL_TABLE_LIST.iter() {
                                        let mtstring = String::from(*mysql_table);

                                        if tables.contains(&mtstring) {
                                            debug!("table: {} missing", mtstring);
                                            create_table(r.clone(), &mtstring);
                                        }
                                    }
                                }

                                RepositoryStartupError::OtherError(ref error) => {
                                    error!(
                                        "Error while starting up repo: {}",
                                        error
                                    );

                                    return Err(repo_startup_error);
                                }

                                RepositoryStartupError::ConnectionError => {
                                    // Does not apply in this case!
                                    // If there's connection error, it will be
                                    // caught by `MySQLRepository::new()`.
                                }
                            }

                            error!("Errors while starting up repository");
                        }
                    }

                    return Ok(Arc::new(r));
                }

                Err(e) => {
                    match e {
                        DBError::ConnectionError => {
                            error!("Please make sure the mysql server is up, check the connection configs and try again.");
                        }

                        // database does not exist
                        DBError::NotFound => {
                            return Err(RepositoryStartupError::DatabaseDoesNotExist);
                        },

                        _ => {}
                    }

                    return Err(RepositoryStartupError::ConnectionError);
                }
            }
        }
    }
}

