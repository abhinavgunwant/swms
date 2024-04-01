use std::result::Result;
use crate::repository::rendition::{
    Rendition, RenditionRepository, Encoding
};
use crate::{
    server::db::DBError,
    db::utils::mysql::{ get_rows_from_query, get_row_from_query },
};
use chrono::{ Local, TimeZone };
use mysql::*;
use mysql::prelude::*;
use log::{ info, debug, error };

fn get_rendition_from_row(row_wrapped: Result<Option<Row>, Error>) -> Result<Rendition, DBError> {
    match row_wrapped {
        Ok (row_option) => {
            match row_option {
                Some (r) => {
                    let mut row: Row = r.clone();

                    let created_on = row.take("CREATED_ON").unwrap();
                    let updated_on = row.take("MODIFIED_ON").unwrap();

                    Ok(Rendition {
                        id: row.take("ID").unwrap(),
                        image_id: row.take("IMAGE_ID").unwrap(),
                        height: row.take("HEIGHT").unwrap(),
                        width: row.take("WIDTH").unwrap(),
                        target_device: row.take("TARGET_DEVICE").unwrap(),
                        slug: row.take("SLUG").unwrap(),
                        is_published: true,
                        encoding: Encoding::JPG,
                        created_on: Local.from_utc_datetime(&created_on).into(),
                        created_by: row.take("CREATED_BY").unwrap(),
                        modified_on: Local.from_utc_datetime(&updated_on).into(),
                        modified_by: row.take("MODIFIED_BY").unwrap(),
                    })
                }

                None => {
                    Err(DBError::NotFound)
                }
            }
        }

        Err (e) => {
            error!("Error while getting rendition from query: {}", e);

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

                let created_on = row.take("CREATED_ON").unwrap();
                let updated_on = row.take("MODIFIED_ON").unwrap();

                renditions.push(Rendition {
                    id: row.take("ID").unwrap(),
                    image_id: row.take("IMAGE_ID").unwrap(),
                    height: row.take("HEIGHT").unwrap(),
                    width: row.take("WIDTH").unwrap(),
                    target_device: row.take("TARGET_DEVICE").unwrap(),
                    slug: row.take("SLUG").unwrap(),
                    is_published: true,
                    encoding: Encoding::JPG,
                    created_on: Local.from_utc_datetime(&created_on).into(),
                    created_by: row.take("CREATED_BY").unwrap(),
                    modified_on: Local.from_utc_datetime(&updated_on).into(),
                    modified_by: 0,
                });
            }

            Ok (renditions)
        }

        Err(e) => {
            error!("Error while getting images from query: {}", e);

            Err(DBError::NotFound)
        }
    }
}

pub struct MySQLRenditionRepository {
    pub connection: PooledConn,
}

impl RenditionRepository for MySQLRenditionRepository {
    fn get(&mut self, id: u32) -> Result<Rendition, DBError> {
        get_rendition_from_row(self.get_row(
            r"SELECT
                ID, IMAGE_ID, HEIGHT, WIDTH, TARGET_DEVICE, SLUG,
                PUBLISHED, CREATED_BY, MODIFIED_BY, CREATED_ON,
                MODIFIED_ON
            FROM IMAGE_RENDITION WHERE ID = :id",
            params! { "id" => id }
        ))
    }

    fn get_from_project_rendition_slug(&mut self, p_slug: String, r_slug: String)
        -> Result<Rendition, DBError> {
        get_rendition_from_row(self.get_row(
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

    fn get_from_folder_rendition_slug(&mut self, f_slug: String, r_slug: String)
        -> Result<Rendition, DBError> {
        get_rendition_from_row(self.get_row(
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

    fn get_from_image_and_slug(&mut self, image_id: u32, slug: String)
        -> Result<Rendition, DBError> {
        get_rendition_from_row(self.get_row(
            r"SELECT
                ID, IMAGE_ID, HEIGHT, WIDTH, TARGET_DEVICE, SLUG, PUBLISHED,
                CREATED_BY, MODIFIED_BY, CREATED_ON, MODIFIED_ON
            FROM IMAGE_RENDITION
            WHERE IMAGE_ID = :image_id AND SLUG = :slug",
            params! { "image_id" => image_id, "slug" => slug }
        ))
    }

    fn get_all(&mut self) -> Result<Vec<Rendition>, DBError> {
        get_renditions_from_row(self.get_rows(
            r"SELECT
                ID, IMAGE_ID, HEIGHT, WIDTH, TARGET_DEVICE, SLUG, PUBLISHED,
                CREATED_BY, MODIFIED_BY, CREATED_ON, MODIFIED_ON
            FROM IMAGE_RENDITION",
            Params::Empty,
        ))
    }

    fn get_all_from_image(&mut self, image_id: u32) -> Result<Vec<Rendition>, DBError> {
        get_renditions_from_row(self.get_rows(
            r"SELECT
                R.ID, R.IMAGE_ID, R.HEIGHT, R.WIDTH, R.TARGET_DEVICE,
                R.SLUG, R.PUBLISHED, R.CREATED_BY, R.MODIFIED_BY,
                R.CREATED_ON, R.MODIFIED_ON 
            FROM IMAGE I, IMAGE_RENDITION R
            WHERE R.IMAGE_ID = I.ID AND I.ID = :image_id",
            params! { "image_id" => image_id }
        ))
    }

    fn get_all_paged(&mut self, _page: u32, _page_length: u32) -> Result<Vec<Rendition>, DBError> {
        // TODO: Implement
        self.get_all()
    }

    fn get_all_from_project(&mut self, project_id: u32) -> Result<Vec::<Rendition>, DBError> {
        get_renditions_from_row(self.get_rows(
            r"SELECT
                R.ID, R.IMAGE_ID, R.HEIGHT, R.WIDTH, R.TARGET_DEVICE, R.SLUG,
                R.PUBLISHED, R.CREATED_BY, R.MODIFIED_BY,
                R.CREATED_ON, R.MODIFIED_ON
            FROM IMAGE_RENDITION R, IMAGE I
            WHERE I.PROJECT_ID = :project_id AND R.IMAGE_ID = I.ID",
            params! { "project_id" => project_id }
        ))
    }

    fn get_all_from_project_slug(&mut self, project_slug: String) -> Result<Vec::<Rendition>, DBError> {
        get_renditions_from_row(self.get_rows(
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

    fn add(&mut self, rendition: Rendition) -> Result<u32, String> {
        debug!("adding a rendition");
        let error_msg: String = String::from("Error Inserting Data!");

        let transaction_result = self.connection.start_transaction(TxOpts::default());

        match transaction_result {
            Ok (mut tx) => {
                let res = tx.exec_drop(
                    r"INSERT INTO IMAGE_RENDITION (
                        IMAGE_ID, HEIGHT, WIDTH, TARGET_DEVICE, SLUG,
                        PUBLISHED, CREATED_BY, MODIFIED_BY, CREATED_ON,
                        MODIFIED_ON
                    ) VALUES (
                        :image_id, :height, :width, :target_device, :slug,
                        :published, :created_by, :modified_by,
                        current_timestamp(), current_timestamp()
                    )",
                    params! {
                        "image_id" => &rendition.image_id,
                        "height" => &rendition.height,
                        "width" => &rendition.width,
                        "target_device" => &rendition.target_device,
                        "slug" => &rendition.slug,
                        "published" => &rendition.is_published,
                        "created_by" => &rendition.created_by,
                        "modified_by" => &rendition.modified_by,
                    }
                );

                match res {
                    Ok (_) => {
                        info!("Rendition data Inserted!");

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

                    Err (e) => {
                        let c_res = tx.rollback();
                        error!("{}", e);

                        match c_res {
                            Ok (_) => {
                                match e {
                                    Error::MySqlError(mysql_error) => {
                                        if mysql_error.code == 1062 {
                                            return Err (String::from("Duplicate slug"));
                                        }

                                        Err(error_msg)
                                    }

                                    _ => Err(error_msg)
                                }
                            }

                            Err (_) => Err(error_msg)
                        }

                        
                    }
                }
            }

            Err (_e) => Err(String::from("Error initializing transaction"))
        }
    }

    fn is_valid_new_slug(&mut self, image_id: u32, slug: String) -> Result<bool, DBError> {
        let row_result: Result<Option<Row>,Error> = self.get_row(
            r"SELECT NOT EXISTS (
                SELECT ID FROM IMAGE_RENDITION WHERE SLUG = :slug
                AND IMAGE_ID = :image_id
            ) AS VALID",
            params! { "slug" => slug, "image_id" => image_id }
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

    fn is_valid_slug(&mut self, image_id: u32, slug: String) -> Result<bool, DBError> {
        match self.is_valid_new_slug(image_id, slug) {
            Ok (valid) => { Ok (!valid) }
            Err (e) => { Err(e) }
        }
    }

    fn update(&mut self, _rendition: Rendition) {
        // TODO: Implement
        debug!("Updating a rendition");
    }

    fn remove(&mut self, rendition: Rendition) -> Result<String, String> {
        debug!("Removing a rendition");
        self.remove_item(rendition.id)
    }

    fn remove_item(&mut self, id: u32) -> Result<String, String> {
        debug!("Removing a rendition item");

        match self.connection.exec_drop(
            r"DELETE FROM IMAGE_RENDITION WHERE ID = :id",
            params! { "id" => id.clone() },
        ) {
            Ok (_) => {
                info!("Rendition removed successfully (ID: {})!", id);

                Ok (String::from("Successfully removed rendition."))
            }

            Err (e) => {
                error!("Error removing rendition (ID: {}): {}", id, e);

                Err (String::from("Unable to remove rendition."))
            }
        }
    }

    fn remove_all_from_image (&mut self, image_id: u32) -> Result<String, String> {
        debug!("Removing all renditions from image: {}", image_id);

        match self.connection.exec_drop(
            r"DELETE FROM IMAGE_RENDITION WHERE IMAGE_ID = :image_id",
            params! { "image_id" => image_id }
        ) {
            Ok (_) => {
                info!("Rendition removed successfully (ID: {})!", image_id);

                Ok (format!("Successfully removed renditions for image with id: {}.", image_id))
            }

            Err (e) => {
                error!("Error removing rendition (ID: {}): {}", image_id, e);

                Err (format!("Unable to remove rendition for image with id: {}", image_id))
            }
        }
    }
}

impl MySQLRenditionRepository {
    fn get_row(&mut self, query: &str, params: Params)
        -> mysql::error::Result<Option<Row>> {
        get_row_from_query(&mut self.connection, query, params)
    }

    fn get_rows(&mut self, query: &str, params: Params)
        -> mysql::error::Result<Vec<Row>> {
        get_rows_from_query(&mut self.connection, query, params)
    }
}

