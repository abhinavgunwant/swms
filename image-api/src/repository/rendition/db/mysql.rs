use std::result::Result;
use crate::repository::rendition::{
    Rendition, RenditionRepository, Encoding
};
use crate::db::{
    DBError, get_db_connection,
    utils::mysql::{ get_rows_from_query, get_row_from_query }
};
use chrono::Utc;
use mysql::*;
use mysql::prelude::*;

fn get_rendition_from_row(row_wrapped: Result<Option<Row>, Error>) -> Result<Rendition, DBError> {
    match row_wrapped {
        Ok (row_option) => {
            match row_option {
                Some (r) => {
                    let mut row: Row = r.clone();

                    Ok(Rendition {
                        id: row.take("ID").unwrap(),
                        image_id: row.take("IMAGE_ID").unwrap(),
                        height: row.take("HEIGHT").unwrap(),
                        width: row.take("WIDTH").unwrap(),
                        target_device: row.take("TARGET_DEVICE").unwrap(),
                        slug: row.take("SLUG").unwrap(),
                        is_published: true,
                        encoding: Encoding::JPG,
                        created_on: Utc::now(),
                        created_by: row.take("CREATED_BY").unwrap(),
                        modified_on: Utc::now(),
                        modified_by: row.take("MODIFIED_BY").unwrap(),
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

fn get_renditions_from_row(row_wrapped: Result<Vec::<Row>, Error>)
    -> Result<Vec::<Rendition>, DBError> {

    match row_wrapped {
        Ok (rows) => {
            let mut renditions: Vec<Rendition> = vec![];

            for row_ in rows.iter() {
                let mut row = row_.clone();

                renditions.push(Rendition {
                    id: row.take("ID").unwrap(),
                    image_id: row.take("IMAGE_ID").unwrap(),
                    height: row.take("HEIGHT").unwrap(),
                    width: row.take("WIDTH").unwrap(),
                    target_device: row.take("TARGET_DEVICE").unwrap(),
                    slug: row.take("SLUG").unwrap(),
                    is_published: true,
                    encoding: Encoding::JPG,
                    created_on: Utc::now(),
                    created_by: row.take("CREATED_BY").unwrap(),
                    modified_on: Utc::now(),
                    modified_by: 0,
                });
            }

            Ok (renditions)
        }

        Err(e) => {
            eprintln!("Error while getting images from query: {}", e);

            Err(DBError::NOT_FOUND)
        }
    }
}

pub struct MySQLRenditionRepository {}

impl RenditionRepository for MySQLRenditionRepository {
    fn get(&self, id: u32) -> Result<Rendition, DBError> {
        get_rendition_from_row(get_row_from_query(
            r"SELECT
                ID, IMAGE_ID, HEIGHT, WIDTH, TARGET_DEVICE, SLUG,
                PUBLISHED, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON
            FROM IMAGE_RENDITION WHERE ID = :id",
            params! { "id" => id }
        ))
    }

    fn get_from_project_rendition_slug(&self, p_slug: String, r_slug: String)
        -> Result<Rendition, DBError> {
        get_rendition_from_row(get_row_from_query(
            r"SELECT
                R.ID, R.IMAGE_ID, R.HEIGHT, R.WIDTH, R.TARGET_DEVICE,
                R.SLUG, R.PUBLISHED, R.CREATED_BY, R.MODIFIED_BY,
                R.CREATED_ON, R.MODIFIED_ON
            FROM IMAGE I, IMAGE_RENDITION R, PROJECT P
            WHERE P.SLUG = :p_slug AND R.SLUG = :r_slug AND I.ID = R.IMAGE_ID
                AND I.PROJECT_ID = P.ID",
            params! { "r_slug" => r_slug, "p_slug" => p_slug }
        ))
    }

    fn get_from_folder_rendition_slug(&self, f_slug: String, r_slug: String)
        -> Result<Rendition, DBError> {
        get_rendition_from_row(get_row_from_query(
            r"SELECT
                IR.ID, IR.IMAGE_ID, IR.HEIGHT, IR.WIDTH, IR.TARGET_DEVICE,
                IR.SLUG, IR.PUBLISHED, IR.CREATED_BY,
                IR.MODIFIED_BY, IR.CREATED_ON, IR.MODIFIED_ON, 
            FROM IMAGE_RENDITION IR, FOLDER F, IMAGE I
            WHERE F.SLUG = :p_slug AND I.SLUG = :r_slug AND I.ID = IR.IMAGE_ID
                AND I.FOLDER_ID = F.ID",
            params! { "r_slug" => r_slug, "f_slug" => f_slug }
        ))
    }

    fn get_all(&self) -> Vec::<Rendition> {
        let mut renditions: Vec::<Rendition> = vec![];

        let rendition: Result<Rendition, DBError> = self.get(0);

        match rendition {
            Ok (ren) => {
                renditions.push(ren);
            }
            Err (_e) => {}
        }

        renditions
    }

    fn get_all_from_image(&self, image_id: u32) -> Result<Vec<Rendition>, DBError> {
        get_renditions_from_row(get_rows_from_query(
            r"SELECT
                R.ID, R.IMAGE_ID, R.HEIGHT, R.WIDTH, R.TARGET_DEVICE,
                R.SLUG, R.PUBLISHED, R.CREATED_BY, R.MODIFIED_BY,
                R.CREATED_ON, R.MODIFIED_ON 
            FROM IMAGE I, IMAGE_RENDITION R
            WHERE R.IMAGE_ID = I.ID AND I.ID = :image_id",
            params! { "image_id" => image_id }
        ))
    }

    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<Rendition> {
        self.get_all()
    }

    fn get_all_from_project(&self, project_id: u32) -> Result<Vec::<Rendition>, DBError> {
        get_renditions_from_row(get_rows_from_query(
            r"SELECT
                R.ID, R.IMAGE_ID, R.HEIGHT, R.WIDTH, R.TARGET_DEVICE, R.SLUG,
                R.PUBLISHED, R.CREATED_BY, R.MODIFIED_BY,
                R.CREATED_ON, R.MODIFIED_ON
            FROM IMAGE_RENDITION R, IMAGE I
            WHERE I.PROJECT_ID = :project_id AND R.IMAGE_ID = I.ID",
            params! { "project_id" => project_id }
        ))
    }

    fn get_all_from_project_slug(&self, project_slug: String) -> Result<Vec::<Rendition>, DBError> {
        get_renditions_from_row(get_rows_from_query(
            r"SELECT
                R.ID, R.IMAGE_ID, R.HEIGHT, R.WIDTH, R.TARGET_DEVICE, R.SLUG,
                R.PUBLISHED, R.CREATED_BY, R.MODIFIED_BY,
                R.CREATED_ON, R.MODIFIED_ON
            FROM IMAGE_RENDITION R, IMAGE I, PROJECT P
            WHERE I.PROJECT_ID = P.ID AND R.IMAGE_ID = I.ID
                AND P.SLUG = :project_slug",
            params! { "project_slug" => project_slug }
        ))
    }

    fn add(&self, rendition: Rendition) {
        println!("adding a rendition");
        let mut conn = get_db_connection();

        conn.exec_drop(
            r"INSERT INTO IMAGE_RENDITION (
                ID, IMAGE_ID, HEIGHT, WIDTH, TARGET_DEVICE, SLUG,
                PUBLISHED, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON
            ) VALUES (
                :id, :image_id, :height, :width, :target_device, :slug,
                :published, :created_by, :modified_by,
                current_timestamp(), current_timestamp()
            )",
            params! {
                "id" => &rendition.id,
                "image_id" => &rendition.image_id,
                "height" => &rendition.height,
                "width" => &rendition.width,
                "target_device" => &rendition.target_device,
                "slug" => &rendition.slug,
                "published" => &rendition.is_published,
                "created_by" => &rendition.created_by,
                "modified_by" => &rendition.modified_by,
            }
        ).expect("Whatever");
    }

    fn update(&self, rendition: Rendition) {
        println!("Updating a rendition");
    }

    fn remove(&self, id: Rendition) {
        println!("Removing a rendition");
    }

    fn remove_item(&self, id: u32) {
        println!("Removing a item");
    }
}
