use chrono::Utc;
use mysql::*;
use mysql::prelude::*;
use std::result::Result;
use crate::{
    db::{
        utils::mysql::{ get_rows_from_query, get_row_from_query },
        DBError, get_db_connection,
    },
    repository::folder::FolderRepository,
    model::folder::Folder,
};

pub struct MySQLFolderRepository {}

fn get_folder_from_row(row_wrapped: Result<Option<Row>, Error>)
    -> std::result::Result<Folder, DBError> {

    match row_wrapped {
        Ok (row_option) => {
            match row_option {
                Some(row_ref) => {
                    let mut row = row_ref.clone();

                    Ok(Folder {
                        id: row.take("ID").unwrap(),
                        title: row.take("TITLE").unwrap(),
                        slug: row.take("SLUG").unwrap(),
                        project_id: row.take("PROJECT_ID").unwrap(),
                        description: row.take("DESCRIPTION").unwrap(),
                        parent_folder_id: row.take("PARENT_FOLDER_ID").unwrap(),
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

fn get_folders_from_row(row_wrapped: Result<Vec<Row>, Error>)
    -> std::result::Result<Vec<Folder>, DBError> {

    match row_wrapped {
        Ok (rows) => {
            let mut folders = vec![];

            for row_ in rows.iter() {
                let mut row = row_.clone();

                let parent_folder_id: u32;
                println!("Getting folder object...");

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

                folders.push(Folder {
                    id: row.take("ID").unwrap(),
                    title: row.take("TITLE").unwrap(),
                    slug: row.take("SLUG").unwrap(),
                    project_id: row.take("PROJECT_ID").unwrap(),
                    description: row.take("DESCRIPTION").unwrap(),
                    parent_folder_id,
                    created_by: row.take("CREATED_BY").unwrap(),
                    modified_by: row.take("MODIFIED_BY").unwrap(),
                    created_on: Utc::now(),
                    modified_on: Utc::now(),
                });
            }

            Ok (folders)
        }

        Err (e) => {
            eprintln!("Error while getting folders from query: {}", e);

            Err(DBError::OtherError)
        }
    }
}

impl FolderRepository for MySQLFolderRepository {
    /**
     * Gets a project based on it's ID.
     */
    fn get(&self, id: u32) -> Result<Folder, DBError> {
        get_folder_from_row(get_row_from_query(
            r"SELECT
                ID, TITLE, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG, PROJECT_ID, PARENT_FOLDER_ID
            FROM FOLDER
            WHERE ID = :id",
            params! { "id" => id },
        ))
    }

    fn get_from_slug(&self, slug: String) -> Result<Folder, DBError> {
        get_folder_from_row(get_row_from_query(
            r"SELECT
                ID, TITLE, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG, PROJECT_ID, PARENT_FOLDER_ID
            FROM FOLDER
            WHERE SLUG = :slug",
            params! { "slug" => slug },
        ))
    }

    fn get_all_from_project(&self, project_id: u32) -> Result<Vec<Folder>, DBError> {
        get_folders_from_row(get_rows_from_query(
            r"SELECT
                ID, TITLE, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG, PROJECT_ID, PARENT_FOLDER_ID
            FROM FOLDER
            WHERE PROJECT_ID = :project_id",
            params! { "project_id" => project_id },
        ))
    }

    fn get_all_from_project_slug(&self, project_slug: String)
        -> Result<Vec<Folder>, DBError> {
        get_folders_from_row(get_rows_from_query(
            r"SELECT
                F.ID, F.TITLE, F.DESCRIPTION, F.CREATED_BY, F.MODIFIED_BY,
                F.CREATED_ON, F.MODIFIED_ON, F.SLUG, F.PROJECT_ID,
                F.PARENT_FOLDER_ID
            FROM FOLDER F, PROJECT P
            WHERE P.SLUG = :project_slug AND F.PROJECT_ID = P.ID",
            params! { "project_slug" => project_slug },
        ))
    }

    fn add(&self, folder: Folder) {

    }

    fn remove(&self, id: u32) {

    }
}

