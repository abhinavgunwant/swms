use crate::repository::item::Item;
use crate::repository::Repository;
use crate::repository::image::{ Image, Encoding, ImageRepository };
use crate::db::dbcontext::DBContext;
use crate::db::{ get_db_context, DBImpl };
use chrono::Utc;
use mysql::*;
use mysql::prelude::*;

const SELECT_ONE: &'static str = r"
    SELECT
        ID, ORIGINAL_FILENAME,  FROM IMAGE";

pub struct ImageRepositoryMySQL {}

// TODO: implement all for MySQL
//impl Repository for ImageRepositoryMySQL {
//    fn get(&self, id: u32) -> Box::<dyn Item> {
//        println!("Getting image with id: {}", id);
//
//        let context = get_db_context();
//
//        println!("DB Context:");
//        println!(use mysql::prelude::Queryable
//            "\tDB Name: {}\n\tConnection String: {}",
//            context.db_name(),
//            context.connection_string()
//        );
//
//        Box::new(Image {
//            name: "test".to_string(),
//            id: 0,
//            encoding: Encoding::JPG,
//            height: 16,
//            width: 16,
//            metadata_id: 0,
//            slug: "test".to_string(),
//            created_by: 0,
//            modified_by: 0,
//            created_on: Utc::now(),
//            modified_on: Utc::now()
//        })
//
//        // let dbc = get_db_context(DBImpl::MYSQL);
//        // dbc
//
//        // Box::new(Image {
//        //     name: String::from_str("test"),
//        //     id: 0,
//        //     encoding: Encoding::JPG,
//        // })
//    }
//
//    fn get_all(&self) -> Vec::<Box<dyn Item>> {
//        println!("Getting all images.");
//
//        let img = Box::new(Image {
//            name: "test".to_string(),
//            id: 0,
//            encoding: Encoding::JPG,
//            height: 16,
//            width: 16,
//            metadata_id: 0,
//            slug: "test".to_string(),
//            created_by: 0,
//            modified_by: 0,
//            created_on: Utc::now(),
//            modified_on: Utc::now()
//        });
//
//        let mut vec = Vec::<Box<dyn Item>>::new();
//        vec.push(img);
//
//        return vec;
//    }
//
//    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<Box::<dyn Item>> {
//        println!("Getting all images, page: {}, page_length: {}.", page, page_length);
//
//        return self.get_all();
//    }
//
//    fn add(&self, item: Box::<dyn Item>) {
//        println!("Adding image item");
//    }
//
//    fn update(&self, item: Box::<dyn Item>) {
//        println!("Updating image item");
//    }
//
//    fn remove_item(&self, item: Box::<dyn Item>) {
//        println!("Removing image item");
//    }
//
//    fn remove(&self, id: u32) {
//        println!("Removing image with id: {}", id);
//    }
//}

pub struct MySQLImageRepository {}

impl ImageRepository for MySQLImageRepository {
    fn get(&self, id: u32) -> Image {
        let dbc:DBContext = get_db_context();

        let pool = Pool::new(String::as_str(&dbc.connection_string));

        let mut conn = pool.unwrap().get_conn().unwrap();

        //let image: &Image = 


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
                    metadata_id: 0,
                }
//            |(id, name, title, height, width, is_published, project_id,
//                folder_id, created_by, modified_by, created_on, modified_on,
//                slug)| {
//                Image {
//                    id: id,
//                    name: String::from(name),
//                    title: String::from(title),
//                    height: height,
//                    width: width,
//                    is_published: is_published == 1,
//                    project_id: project_id,
//                    folder_id: folder_id,
//                    created_by: created_by,
//                    modified_by: modified_by,
//                    created_on: created_on,
//                    modified_on: modified_on,
//                    slug: slug,
//                    encoding: Encoding::JPG,
//                    metadata_id: 0
//                }
            }
        ).unwrap();

        //let image: Image = 

        let img: &Image = images.get(0).unwrap();

        //*image

//        Image {
//            name: String::from("test"),
//            id: 0,
//            encoding: Encoding::JPG,
//            height: 0,
//            width: 0,
//            metadata_id: 0,
//            slug: String::from("test"),
//            created_on: Utc::now(),
//            created_by: 0,
//            modified_on: Utc::now(),
//            modified_by: 0
//        }
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
        println!("ading an image");
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

