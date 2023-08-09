use std::result::Result;
use chrono::Utc;
use mysql::*;
use mysql::prelude::*;

use crate::{
    repository::image::{ Encoding, ImageRepository },
    db::{
        utils::mysql::{
            get_rows_from_query, get_row_from_query, process_id_from_row_result
        },
        DBError, get_db_connection
    },
    model::image::Image,
};

fn get_image_from_row (row_wrapped: Result<Option<Row>, Error>) -> Result<Image, DBError> {
    match row_wrapped {
        Ok (row_option) => {
            match row_option {
                Some (r) => {
                    let mut row: Row = r.clone();

                    Ok (Image {
                        id: row.take("ID").unwrap(),
                        name: row.take("ORIGINAL_FILENAME").unwrap(),
                        title: row.take("TITLE").unwrap(),
                        height: row.take("HEIGHT").unwrap(),
                        width: row.take("WIDTH").unwrap(),
                        is_published: true,
                        // is_published: row.take("is_published").unwrap() == true,
                        project_id: row.take("PROJECT_ID").unwrap_or_default(),
                        folder_id: row.take("FOLDER_ID").unwrap_or_default(),
                        created_by: row.take("CREATED_BY").unwrap(),
                        modified_by: row.take("MODIFIED_BY").unwrap(),
                        created_on: Utc::now(),
                        // created_on: row.take("created_on").unwrap(),
                        modified_on: Utc::now(),
                        // modified_on: row.take("modified_on").unwrap(),
                        encoding: Encoding::JPG,
                        //metadata_id: 0,
                    })
                }

                None => {
                    Err(DBError::NOT_FOUND)
                }
            }
        }

        Err (_e) => {
            eprintln!("Error while getting images from query: {}", _e);

            Err(DBError::OtherError)
        }
    }
}

fn get_images_from_row(row_wrapped: Result<Vec::<Row>, Error>)
    -> Result<Vec::<Image>, DBError> {

    match row_wrapped {
        Ok (rows) => {
            let mut images = vec![];

            for row_ in rows.iter() {
                let mut row = row_.clone();

                let folder_id: u32;

                println!("Getting image object...");

                match row.take_opt("FOLDER_ID") {
                    Some (fi_result) => {
                        match fi_result {
                            Ok (fi) => {
                                folder_id = fi
                            }

                            Err (_e) => {
                                folder_id = 0;
                            }
                        }
                    }
                    None => folder_id = 0
                }

                images.push(Image {
                    id: row.take("ID").unwrap(),
                    name: row.take("ORIGINAL_FILENAME").unwrap(),
                    title: row.take("TITLE").unwrap(),
                    encoding: Encoding::JPG,
                    height: row.take("HEIGHT").unwrap(),
                    width: row.take("WIDTH").unwrap(),
                    is_published: true,
                    project_id: row.take("PROJECT_ID").unwrap(),
                    // folder_id: row.take("FOLDER_ID").unwrap(),
                    folder_id,
                    //metadata_id: 0,
                    created_by: row.take("CREATED_BY").unwrap(),
                    modified_by: 0,
                    created_on: Utc::now(),
                    modified_on: Utc::now(),
                });
            }

            Ok (images)
        }

        Err(e) => {
            eprintln!("Error while getting images from query: {}", e);

            Err(DBError::NOT_FOUND)
        }
    }
}

pub struct MySQLImageRepository {}

impl ImageRepository for MySQLImageRepository {
    fn get(&self, id: u32) -> Result<Image, DBError> {
        get_image_from_row(get_row_from_query(
            r"SELECT
                ID, ORIGINAL_FILENAME, TITLE, HEIGHT, WIDTH, PUBLISHED,
                PROJECT_ID, FOLDER_ID, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON
            FROM IMAGE WHERE ID = :id",
            params! { "id" => id },
        ))
    }

    /**
     * Gets a project from it's slug.
     */
    fn get_from_slug(&self, slug: &str) -> Result<Image, DBError> {
        get_image_from_row(get_row_from_query(
            r"SELECT
                ID, ORIGINAL_FILENAME, TITLE, HEIGHT, WIDTH, PUBLISHED,
                PROJECT_ID, FOLDER_ID, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON
            FROM IMAGE WHERE ID = :slug",
            params! {"slug" => slug},
        ))
    }

    fn get_all(&self) -> Result<Vec<Image>, DBError> {
        get_images_from_row(get_rows_from_query(
            r"SELECT
                ID, ORIGINAL_FILENAME, TITLE, HEIGHT, WIDTH, PUBLISHED,
                PROJECT_ID, FOLDER_ID, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON
            FROM IMAGE",
            Params::Empty,
        ))
    }

    fn get_all_paged(&self, page: u32, page_length: u32) -> Result<Vec<Image>, DBError> {
        self.get_all()
    }

    fn get_all_from_project(&self, project_id: u32) -> Result<Vec::<Image>, DBError> {
        get_images_from_row(get_rows_from_query(
            r"SELECT
                ID, ORIGINAL_FILENAME, TITLE, HEIGHT, WIDTH, PUBLISHED,
                PROJECT_ID, FOLDER_ID, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON
            FROM IMAGE WHERE PROJECT_ID = :project_id",
            params! { "project_id" => project_id }
        ))
    }

    fn get_from_project_slug(&self, project_slug: String, all: bool)
        -> Result<Vec::<Image>, DBError> {
        get_images_from_row(get_rows_from_query(
            format!(
                r"SELECT
                    I.ID, I.ORIGINAL_FILENAME, I.TITLE, I.HEIGHT, I.WIDTH,
                    I.PUBLISHED, I.PROJECT_ID, I.FOLDER_ID, I.CREATED_BY,
                    I.MODIFIED_BY, I.CREATED_ON, I.MODIFIED_ON
                FROM IMAGE I, PROJECT P
                WHERE I.PROJECT_ID = P.ID AND P.SLUG = :project_slug {}",
                if all { "" } else { " AND I.FOLDER_ID = 0" },
            ).as_str(),
            params! { "project_slug" => project_slug }
        ))
    }

    fn get_from_folder_slug(&self, folder_slug: String, all: bool)
            -> Result<Vec<Image>, DBError> {
        get_images_from_row(get_rows_from_query(
            r"SELECT
                I.ID, I.ORIGINAL_FILENAME, I.TITLE, I.HEIGHT, I.WIDTH,
                I.PUBLISHED, I.PROJECT_ID, I.FOLDER_ID, I.CREATED_BY,
                I.MODIFIED_BY, I.CREATED_ON, I.MODIFIED_ON
            FROM IMAGE I, FOLDER F
            WHERE I.FOLDER_ID = F.ID AND F.SLUG = :folder_slug",
            params! { "folder_slug" => folder_slug }
        ))
    }

    fn add(&self, image: Image) -> Result<u32, String> {
        println!("adding an image");

        let error_msg: String = String::from("Error Inserting Data!");

        let mut conn = get_db_connection();
        let transaction_result = conn.start_transaction(TxOpts::default());

        match transaction_result {
            Ok (mut tx) => {
                let res = tx.exec_drop(
                    r"INSERT INTO IMAGE (
                        ID, ORIGINAL_FILENAME, TITLE, HEIGHT, WIDTH, PUBLISHED,
                        PROJECT_ID, FOLDER_ID, CREATED_BY, MODIFIED_BY, CREATED_ON,
                        MODIFIED_ON
                    ) VALUES (
                        :id, :original_filename, :title, :height, :width, :published,
                        :project_id, :folder_id, :created_by, :modified_by,
                        current_timestamp(), current_timestamp()
                    )",
                    params! {
                        "id" => &image.id,
                        "original_filename" => &image.name,
                        "title" => &image.title,
                        "height" => &image.height,
                        "width" => &image.width,
                        "published" => &image.is_published,
                        "project_id" => &image.project_id,
                        "folder_id" => &image.folder_id,
                        "created_by" => &image.created_by,
                        "modified_by" => &image.modified_by,
                    }
                );

                match res {
                    Ok (_) => {
                        println!("Data Inserted!");

                        let row_wrapped: Result<Option<Row>, Error> = tx.exec_first(
                            r"SELECT LAST_INSERT_ID() as LID;",
                            Params::Empty,
                        );

                        match row_wrapped {
                            Ok(row_option) => {
                                match row_option {
                                    Some (mut row) => {
                                        match row.take("LID") {
                                            Some (id) => {
                                                let c_res = tx.commit();
                                                
                                                match c_res {
                                                    Ok (_) => Ok(id),
                                                    Err (_) => Err(error_msg)
                                                }
                                            }

                                            None => {
                                                let c_res = tx.rollback();
                                                
                                                match c_res {
                                                    Ok (_) => Err(error_msg),
                                                    Err (_) => Err(error_msg)
                                                }
                                            }
                                        }
                                    }

                                    None => {
                                        let c_res = tx.rollback();
                                        
                                        match c_res {
                                            Ok (_) => Err(error_msg),
                                            Err (_) => Err(error_msg)
                                        }
                                    }
                                }
                            }

                            Err(_e) => {
                                let c_res = tx.rollback();
                                
                                match c_res {
                                    Ok (_) => Err(error_msg),
                                    Err (_) => Err(error_msg)
                                }
                            }
                        }
                    }

                    Err (_) => {
                        let c_res = tx.rollback();
                        
                        match c_res {
                            Ok (_) => Err(error_msg),
                            Err (_) => Err(error_msg)
                        }
                    }
                }
            }

            Err (_e) => Err(String::from("Error initializing transaction"))
        }
    }

    fn is_valid_new_slug(&self, slug: String) -> Result<bool, DBError> {
        let row_result: Result<Option<Row>,Error> = get_row_from_query(
            r"SELECT NOT EXISTS (
                SELECT ID FROM IMAGE WHERE SLUG = :slug
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
        process_id_from_row_result(get_row_from_query(
            "SELECT ID FROM IMAGE WHERE SLUG = :slug",
            params! { "slug" => slug }
        ))
    }

    fn update(&self, image: Image) -> Result<String, String>{
        let mut conn = get_db_connection();

        println!("Updating an image");

        match conn.exec_drop(r"UPDATE IMAGE SET
                ORIGINAL_FILENAME = :original_filename, TITLE = :title,
                HEIGHT = :height, WIDTH = :width, PUBLISHED = :published,
                PROJECT_ID = :project_id, FOLDER_ID = :folder_id,
                MODIFIED_BY = :modified_by, MODIFIED_ON = current_timestamp()
            WHERE ID = :id",
            params! {
                "id" => &image.id,
                "original_filename" => &image.name,
                "title" => &image.title,
                "height" => &image.height,
                "width" => &image.width,
                "published" => &image.is_published,
                "project_id" => &image.project_id,
                "folder_id" => &image.folder_id,
                "modified_by" => &image.modified_by,
            }
        ) {
            Ok(_) => Ok(String::from("Successfully updated image!")),

            Err (e) => {
                eprintln!("Error while updating image: {}", e);

                Err(String::from("Unable to update image."))
            }
        }
    }

    fn remove(&self, image: Image) -> Result<String, String> {
        println!("Removing an image");

        self.remove_item(image.id)
    }

    fn remove_item(&self, id: u32) -> Result<String, String> {
        println!("Removing an image item");
        let mut conn = get_db_connection();

        match conn.exec_drop(
            r"DELETE FROM IMAGE WHERE ID = :id",
            params! { "id" => id.clone() },
        ) {
            Ok (_) => {
                println!("Image with ID: {} removed successfully!", id);

                Ok (String::from("Successfully removed image."))
            }

            Err (e) => {
                eprintln!("Unable to remove image with ID: {}\nError: {}", id, e);

                Err (String::from("Unable to remove image."))
            }
        }
    }
}

