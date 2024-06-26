use chrono::{ Local, TimeZone };
use mysql::*;
use mysql::prelude::*;
use std::result::Result;
use log::{ info, debug, error };

use crate::{
    server::db::DBError, db::utils::mysql::{
        get_rows_from_query, get_row_from_query, process_id_from_row_result
    },
    repository::folder::FolderRepository, model::folder::Folder,
};

pub struct MySQLFolderRepository {
    pub connection: PooledConn,
}

fn get_folder_from_row(row_wrapped: Result<Option<Row>, Error>)
    -> std::result::Result<Folder, DBError> {

    match row_wrapped {
        Ok (row_option) => {
            match row_option {
                Some(row_ref) => {
                    let mut row = row_ref.clone();

                    let parent_folder_id: u32;
                    let modified_by: u32;
                    debug!("Getting folder object...");

                    match row.take_opt("PARENT_FOLDER_ID") {
                        Some (fi_result) => {
                            match fi_result {
                                Ok (fi) => {
                                    parent_folder_id = fi
                                }

                                Err (_e) => {
                                    parent_folder_id = 0;
                                }
                            }
                        }
                        None => parent_folder_id = 0
                    }

                    match row.take_opt("MODIFIED_BY") {
                        Some (fi_result) => {
                            match fi_result {
                                Ok (fi) => {
                                    modified_by = fi
                                }

                                Err (_e) => {
                                    modified_by = 0;
                                }
                            }
                        }
                        None => modified_by = 0
                    }

                    let created_on = row.take("CREATED_ON").unwrap();
                    let updated_on = row.take("MODIFIED_ON").unwrap();

                    Ok(Folder {
                        id: row.take("ID").unwrap(),
                        title: row.take("TITLE").unwrap(),
                        slug: row.take("SLUG").unwrap(),
                        project_id: row.take("PROJECT_ID").unwrap(),
                        description: row.take("DESCRIPTION").unwrap(),
                        parent_folder_id,
                        created_by: row.take("CREATED_BY").unwrap(),
                        modified_by,
                        created_on: Local.from_utc_datetime(&created_on).into(),
                        modified_on: Local.from_utc_datetime(&updated_on).into(),
                    })
                }

                None => {
                    Err(DBError::NotFound)
                }
            }
        }

        Err (e) => {
            debug!("Error while getting rendition from query: {}", e);

            Err(DBError::OtherError)
        }
    }
}

fn get_folders_from_row(row_wrapped: Result<Vec<Row>, Error>)
    -> std::result::Result<Vec<Folder>, DBError> {

    match row_wrapped {
        Ok (rows) => {
            let mut folders = vec![];

            for row_ in rows.iter() {
                let mut row = row_.clone();

                let parent_folder_id: u32;
                let modified_by: u32;
                debug!("Getting folder object...");

                match row.take_opt("PARENT_FOLDER_ID") {
                    Some (fi_result) => {
                        match fi_result {
                            Ok (fi) => {
                                parent_folder_id = fi
                            }

                            Err (_e) => {
                                parent_folder_id = 0;
                            }
                        }
                    }
                    None => parent_folder_id = 0
                }

                match row.take_opt("MODIFIED_BY") {
                    Some (fi_result) => {
                        match fi_result {
                            Ok (fi) => {
                                modified_by = fi
                            }

                            Err (_e) => {
                                modified_by = 0;
                            }
                        }
                    }
                    None => modified_by = 0
                }

                let created_on = row.take("CREATED_ON").unwrap();
                let updated_on = row.take("MODIFIED_ON").unwrap();

                folders.push(Folder {
                    id: row.take("ID").unwrap(),
                    title: row.take("TITLE").unwrap(),
                    slug: row.take("SLUG").unwrap(),
                    project_id: row.take("PROJECT_ID").unwrap(),
                    description: row.take("DESCRIPTION").unwrap(),
                    parent_folder_id,
                    created_by: row.take("CREATED_BY").unwrap(),
                    modified_by,
                    created_on: Local.from_utc_datetime(&created_on).into(),
                    modified_on: Local.from_utc_datetime(&updated_on).into(),
                });
            }

            Ok (folders)
        }

        Err (e) => {
            debug!("Error while getting folders from query: {}", e);

            Err(DBError::OtherError)
        }
    }
}

impl FolderRepository for MySQLFolderRepository {
    /**
     * Gets a project based on it's ID.
     */
    fn get(&mut self, id: u32) -> Result<Folder, DBError> {
        get_folder_from_row(self.get_row(
            r"SELECT
                ID, TITLE, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG, PROJECT_ID, PARENT_FOLDER_ID
            FROM FOLDER
            WHERE ID = :id",
            params! { "id" => id },
        ))
    }

    fn get_from_slug(&mut self, slug: String) -> Result<Folder, DBError> {
        get_folder_from_row(self.get_row(
            r"SELECT
                ID, TITLE, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG, PROJECT_ID, PARENT_FOLDER_ID
            FROM FOLDER
            WHERE SLUG = :slug",
            params! { "slug" => slug },
        ))
    }

    fn get_all_from_project(&mut self, project_id: u32) -> Result<Vec<Folder>, DBError> {
        get_folders_from_row(self.get_rows(
            r"SELECT
                ID, TITLE, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG, PROJECT_ID, PARENT_FOLDER_ID
            FROM FOLDER
            WHERE PROJECT_ID = :project_id",
            params! { "project_id" => project_id },
        ))
    }

    fn get_from_project_slug(&mut self, project_slug: String, all: bool)
        -> Result<Vec<Folder>, DBError> {
        get_folders_from_row(self.get_rows(
            format!(
                r"SELECT
                    F.ID, F.TITLE, F.DESCRIPTION, F.CREATED_BY, F.MODIFIED_BY,
                    F.CREATED_ON, F.MODIFIED_ON, F.SLUG, F.PROJECT_ID,
                    F.PARENT_FOLDER_ID
                FROM FOLDER F, PROJECT P
                WHERE P.SLUG = :project_slug AND F.PROJECT_ID = P.ID {}",
                if all { "" } else { "AND F.PARENT_FOLDER_ID = 0" }
            ).as_str(),
            params! { "project_slug" => project_slug },
        ))
    }

    fn get_from_folder(&mut self, folder_id: u32) -> Result<Vec<Folder>, DBError> {
        get_folders_from_row(self.get_rows(
            format!(
                r"SELECT
                    F.ID, F.TITLE, F.DESCRIPTION, F.CREATED_BY, F.MODIFIED_BY,
                    F.CREATED_ON, F.MODIFIED_ON, F.SLUG, F.PROJECT_ID,
                    F.PARENT_FOLDER_ID
                FROM FOLDER F, FOLDER F2
                WHERE F2.ID = :folder_id AND F.PARENT_FOLDER_ID = F2.ID",
            ).as_str(),
            params! { "folder_id" => folder_id },
        ))
    }

    fn get_from_folder_slug(&mut self, folder_slug: String, _all: bool)
            -> Result<Vec<Folder>, DBError> {
        get_folders_from_row(self.get_rows(
            format!(
                r"SELECT
                    F.ID, F.TITLE, F.DESCRIPTION, F.CREATED_BY, F.MODIFIED_BY,
                    F.CREATED_ON, F.MODIFIED_ON, F.SLUG, F.PROJECT_ID,
                    F.PARENT_FOLDER_ID
                FROM FOLDER F, FOLDER F2
                WHERE F2.SLUG = :folder_slug AND F.PARENT_FOLDER_ID = F2.ID",
            ).as_str(),
            params! { "folder_slug" => folder_slug },
        ))
    }

    fn add(&mut self, folder: Folder) -> Result<String, String> {
        match self.connection.exec_drop(
            r"INSERT INTO FOLDER (
                TITLE, DESCRIPTION, SLUG, PROJECT_ID, PARENT_FOLDER_ID,
                CREATED_BY, MODIFIED_BY, CREATED_ON, MODIFIED_ON
            ) VALUES (
                :title, :description, :slug, :project_id, :parent_folder_id,
                :created_by, NULL, CURRENT_TIMESTAMP(), NULL
            )",
            params! {
                "title" => &folder.title,
                "description" => &folder.description,
                "slug" => &folder.slug,
                "project_id" => &folder.project_id,
                "parent_folder_id" => &folder.parent_folder_id,
                "created_by" => &folder.created_by,
            }
        ) {
            Ok (_) => Ok(String::from("Successfully created new folder!")),

            Err (e) => {
                error!("Error inserting new folder: {}", e);

                match e {
                    Error::MySqlError (mysql_error) => {
                        if mysql_error.code == 1062 {
                            return Err(format!(
                                "A folder already exists with slug '{}'",
                                &folder.slug,
                            ));
                        }
                    }

                    _ => {}
                }

                Err(String::from("Error creating new folder!"))
            }
        }
    }

    fn is_valid_new_slug(&mut self, slug: String) -> Result<bool, DBError> {
        let row_result: Result<Option<Row>,Error> = self.get_row(
            r"SELECT NOT EXISTS (
                SELECT ID FROM FOLDER WHERE SLUG = :slug
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

    fn is_valid_slug(&mut self, project_id: u32, folder_id: u32, slug: String) ->
        Result<Option<u32>, DBError> {
        process_id_from_row_result(self.get_row(
            "SELECT ID FROM FOLDER WHERE SLUG = :slug AND
            PROJECT_ID = :project_id AND PARENT_FOLDER_ID = :folder_id",
            params! {
                "project_id" => project_id,
                "folder_id" => folder_id,
                "slug" => slug,
            }
        ))
    }

    fn update(&mut self, folder: Folder) -> Result<String, String> {
        debug!("Updating a folder");

        match self.connection.exec_drop(r"UPDATE FOLDER SET
                TITLE = :title, SLUG = :slug, DESCRIPTION = :description,
                PROJECT_ID = :project_id, PARENT_FOLDER_ID = :parent_folder_id,
                MODIFIED_BY = :modified_by, MODIFIED_ON = current_timestamp()
            WHERE ID = :id",
            params! {
                "id" => &folder.id,
                "slug" => &folder.slug,
                "description" => &folder.description,
                "title" => &folder.title,
                "project_id" => &folder.project_id,
                "parent_folder_id" => &folder.parent_folder_id,
                "modified_by" => &folder.modified_by,
            }
        ) {
            Ok(_) => Ok(String::from("Successfully updated folder!")),

            Err (e) => {
                error!("Error while updating folder: {}", e);

                Err(String::from("Unable to update folder."))
            }
        }
    }

    fn remove(&mut self, folder: Folder) -> Result<String, String> {
        self.remove_item(folder.id)
    }

    fn remove_item(&mut self, folder_id: u32) -> Result<String, String> {
        debug!("Removing a folder item");

        match self.connection.exec_drop(
            r"DELETE FROM FOLDER WHERE ID = :id",
            params! { "id" => folder_id.clone() },
        ) {
            Ok (_) => {
                info!("Folder removed successfully (ID: {})!", folder_id);

                Ok (String::from("Successfully removed folder."))
            }

            Err (e) => {
                error!("Error removing folder (ID: {}): {}", folder_id, e);

                Err (String::from("Unable to remove folder."))
            }
        }
    }
}

impl MySQLFolderRepository {
    fn get_row(&mut self, query: &str, params: Params)
        -> mysql::error::Result<Option<Row>> {
        get_row_from_query(&mut self.connection, query, params)
    }

    fn get_rows(&mut self, query: &str, params: Params)
        -> mysql::error::Result<Vec<Row>> {
        get_rows_from_query(&mut self.connection, query, params)
    }
}

