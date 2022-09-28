use crate::repository::image::{ Image, Encoding, ImageRepository };
use crate::db::dbcontext::DBContext;
use crate::db::get_db_context;
use chrono::Utc;
use mysql::*;
use mysql::prelude::*;

const SELECT_ONE: &'static str = r"
    SELECT
        ID, ORIGINAL_FILENAME, TITLE, HEIGHT, WIDTH, PUBLISHED, PROJECT_ID,
        FOLDER_ID, CREATED_BY, MODIFIED_BY, CREATED_ON, MODIFIED_ON, SLUG
    FROM IMAGE";

const ADD_ONE: &'static str = r"
    INSERT INTO IMAGE (
        ID, ORIGINAL_FILENAME, TITLE, HEIGHT, WIDTH, PUBLISHED, PROJECT_ID,
        FOLDER_ID, CREATED_BY, MODIFIED_BY, CREATED_ON, MODIFIED_ON, SLUG
    ) VALUES (
        :id, :original_filename, :title, :height, :width, :published,
        :project_id, :folder_id, :created_by, :modified_by, current_timestamp(),
        current_timestamp(), :slug
    )
";

pub struct MySQLImageRepository {}

impl ImageRepository for MySQLImageRepository {
    fn get(&self, id: u32) -> Image {
        let dbc:DBContext = get_db_context();

        let pool = Pool::new(String::as_str(&dbc.connection_string));

        let mut conn = pool.unwrap().get_conn().unwrap();

        let images = conn.query_map(SELECT_ONE, |mut row: Row| {
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
                metadata_id: 0,
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
            metadata_id: 0,
        }
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

    fn add(&self, image: Image) {
        println!("adding an image");

        let dbc:DBContext = get_db_context();

        let pool = Pool::new(String::as_str(&dbc.connection_string));

        let mut conn = pool.unwrap().get_conn().unwrap();

        conn.exec_drop(ADD_ONE, params! {
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
        }).expect("Whatever");
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
