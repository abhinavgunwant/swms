use chrono::Utc;
use mysql::*;
use mysql::prelude::*;
use std::result::Result;
use crate::db::{ DBError, get_db_connection };
use crate::repository::folder::{ Folder, FolderRepository };

pub struct MySQLFolderRepository {}

fn get_folder_from_row(row_wrapped: Option<&Row>)
    -> std::result::Result<Folder, DBError> {

    match row_wrapped {
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

impl FolderRepository for MySQLFolderRepository {
    /**
     * Gets a project based on it's ID.
     */
    fn get(&self, id: u32) -> Result<Folder, DBError> {
        let mut conn: PooledConn = get_db_connection();

        let statement = conn.prep(
            r"SELECT ID, TITLE, DESCRIPTION, CREATED_BY, MODIFIED_BY, CREATED_ON,
            MODIFIED_ON, SLUG, PROJECT_ID, PARENT_FOLDER_ID WHERE ID = :id"
        ).unwrap();

        let rows: Vec<Row> = conn.exec(statement, params! {"id" => id}).unwrap();
        get_folder_from_row(rows.get(0))
    }

    fn get_from_slug(&self, slug: String) -> Result<Folder, DBError> {
        self.get(0)
    }

    fn add(&self, folder: Folder) {

    }

    fn remove(&self, id: u32) {

    }
}
