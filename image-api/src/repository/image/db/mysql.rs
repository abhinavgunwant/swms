use std::result::Result;
use chrono::Utc;
use mysql::*;
use mysql::prelude::*;

use crate::{
    repository::image::{ Encoding, ImageRepository },
    db::{
        utils::mysql::{ get_rows_from_query, get_row_from_query },
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
                        project_id: 0,
                        // project_id: row.take("PROJECT_ID").unwrap_or_default(),
                        folder_id: 0,
                        // folder_id: row.take("FOLDER_ID").unwrap_or_default(),
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

                let mut folder_id: u32;

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
                    folder_id: folder_id,
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

    fn get_all_from_project_slug(&self, project_slug: String) -> Result<Vec::<Image>, DBError> {
        get_images_from_row(get_rows_from_query(
            r"SELECT
                I.ID, I.ORIGINAL_FILENAME, I.TITLE, I.HEIGHT, I.WIDTH,
                I.PUBLISHED, I.PROJECT_ID, I.FOLDER_ID, I.CREATED_BY,
                I.MODIFIED_BY, I.CREATED_ON, I.MODIFIED_ON
            FROM IMAGE I, PROJECT P
            WHERE I.PROJECT_ID = P.ID AND P.SLUG = :project_slug",
            params! { "project_slug" => project_slug }
        ))
    }

    fn add(&self, image: Image) -> bool {
        println!("adding an image");

        let mut conn = get_db_connection();
        let res = conn.exec_drop(
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
            Ok (_) => true,
            Err (_) => false,
        }
    }

    fn update(&self, image: Image) {
        println!("Updating an image");
    }

    fn remove(&self, id: Image) {
        println!("Removing an image");
    }

    fn remove_item(&self, id: u32) {
        println!("Removing an image item");
    }
}
