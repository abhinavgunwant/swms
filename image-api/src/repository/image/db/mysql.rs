use std::result::Result;
use crate::repository::image::{ Image, Encoding, ImageRepository };
use crate::db::dbcontext::DBContext;
use crate::db::{ DBError, get_db_connection, get_db_context };
use chrono::Utc;
use mysql::*;
use mysql::prelude::*;

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
                        slug: String::from(""),
                        //slug: row.take("SLUG").unwrap_or_default(),
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
                    slug: row.take("SLUG").unwrap(),
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
    fn get(&self, id: u32) -> Image {
        let dbc:DBContext = get_db_context();

        let pool = Pool::new(String::as_str(&dbc.connection_string));

        let mut conn = pool.unwrap().get_conn().unwrap();

        let images = conn.query_map(
            r"SELECT
                ID, ORIGINAL_FILENAME, TITLE, HEIGHT, WIDTH, PUBLISHED,
                PROJECT_ID, FOLDER_ID, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG
            FROM IMAGE",
            |mut row: Row| {
                Image {
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
                    slug: String::from(""),
                    //slug: row.take("SLUG").unwrap_or_default(),
                    encoding: Encoding::JPG,
                    //metadata_id: 0,
                }
        }).unwrap();


        let img: &Image = images.get(0).unwrap();

        // TODO: implement Copy trait on Image so that this line won't be required
        Image {
            id: img.id,
            name: img.name.clone(),
            title: img.title.clone(),
            height: img.height,
            width: img.width,
            is_published: true,
            // is_published: row.take("is_published").unwrap() == true,
            project_id: img.project_id,
            folder_id: img.folder_id,
            created_by: img.created_by,
            modified_by: img.modified_by,
            created_on: Utc::now(),
            // created_on: row.take("created_on").unwrap(),
            modified_on: Utc::now(),
            // modified_on: row.take("modified_on").unwrap(),
            slug: img.slug.clone(),
            encoding: Encoding::JPG,
            //metadata_id: 0,
        }
    }

    fn get_from_project_image_slug(&self, p_slug: String, i_slug: String)
        -> Result<Image, DBError> {
        let dbc:DBContext = get_db_context();

        let pool = Pool::new(String::as_str(&dbc.connection_string));

        let mut conn = pool.unwrap().get_conn().unwrap();

        let row: Result<Option<Row>, Error> = conn.exec_first(
            r"SELECT
                I.ID, I.ORIGINAL_FILENAME, I.TITLE, I.HEIGHT, I.WIDTH,
                I.PUBLISHED, I.PROJECT_ID, I.FOLDER_ID, I.CREATED_BY,
                I.MODIFIED_BY, I.CREATED_ON, I.MODIFIED_ON, I.SLUG
            FROM IMAGE I, PROJECT P
            WHERE P.SLUG = :p_slug AND I.SLUG = :i_slug
                AND I.PROJECT_ID = P.ID",
            params! {
                "i_slug" => i_slug,
                "p_slug" => p_slug,
            });

        get_image_from_row(row)
    }

    fn get_from_folder_image_slug(&self, f_slug: String, i_slug: String)
        -> Result<Image, DBError> {
        let dbc:DBContext = get_db_context();

        let pool = Pool::new(String::as_str(&dbc.connection_string));

        let mut conn = pool.unwrap().get_conn().unwrap();

        let row: Result<Option<Row>, Error> = conn.exec_first(
            r"SELECT
                I.ID, I.ORIGINAL_FILENAME, I.TITLE, I.HEIGHT, I.WIDTH,
                I.PUBLISHED, I.PROJECT_ID, I.FOLDER_ID, I.CREATED_BY,
                I.MODIFIED_BY, I.CREATED_ON, I.MODIFIED_ON, I.SLUG
            FROM IMAGE I, FOLDER F
            WHERE F.SLUG = :p_slug AND I.SLUG = :i_slug
                AND I.FOLDER_ID = F.ID",
            params! {
                "i_slug" => i_slug,
                "f_slug" => f_slug,
            });

        get_image_from_row(row)
    }

    fn get_all(&self) -> Vec::<Image> {
        let image = self.get(0);

        let mut images = Vec::new();
        images.push(image);

        images
    }

    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<Image> {
        self.get_all()
    }

    fn get_all_from_project(&self, project_id: u32) -> Result<Vec::<Image>, DBError> {
        let mut conn: PooledConn = get_db_connection();

        let statement = conn.prep(
            r"SELECT
                ID, ORIGINAL_FILENAME, TITLE, HEIGHT, WIDTH, PUBLISHED,
                PROJECT_ID, FOLDER_ID, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG
            FROM IMAGE WHERE PROJECT_ID = :project_id
        ").unwrap();

        let rows_wrapped: Result<Vec::<Row>, Error> = conn.exec(
            statement,
            params! { "project_id" => project_id }
        );

        get_images_from_row(rows_wrapped)
    }

    fn get_all_from_project_slug(&self, project_slug: String) -> Result<Vec::<Image>, DBError> {
        let mut conn: PooledConn = get_db_connection();

        let statement = conn.prep(
            r"SELECT
                I.ID, I.ORIGINAL_FILENAME, I.TITLE, I.HEIGHT, I.WIDTH,
                I.PUBLISHED, I.PROJECT_ID, I.FOLDER_ID, I.CREATED_BY,
                I.MODIFIED_BY, I.CREATED_ON, I.MODIFIED_ON, I.SLUG
            FROM IMAGE I, PROJECT P
            WHERE I.PROJECT_ID = P.ID AND P.SLUG = :project_slug
        ").unwrap();

        let rows_wrapped: Result<Vec::<Row>, Error> = conn.exec(
            statement,
            params! { "project_slug" => project_slug }
        );

        get_images_from_row(rows_wrapped)
    }

    fn add(&self, image: Image) {
        println!("adding an image");

        let dbc:DBContext = get_db_context();

        let pool = Pool::new(String::as_str(&dbc.connection_string));

        let mut conn = pool.unwrap().get_conn().unwrap();

        conn.exec_drop(
            r"INSERT INTO IMAGE (
                ID, ORIGINAL_FILENAME, TITLE, HEIGHT, WIDTH, PUBLISHED,
                PROJECT_ID, FOLDER_ID, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON, SLUG
            ) VALUES (
                :id, :original_filename, :title, :height, :width, :published,
                :project_id, :folder_id, :created_by, :modified_by,
                current_timestamp(), current_timestamp(), :slug
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
                "slug" => &image.slug
            }
        ).expect("Whatever");
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
