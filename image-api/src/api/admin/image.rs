use actix_web::{
    web::{ Json, block }, HttpResponse, HttpRequest, post, get, put, delete,
};
use std::fs::{ read, rename, remove_file };
use serde::{ Serialize, Deserialize };
use chrono::Utc;
use crate::{
    db::DBError,
    repository::{
        image::{ ImageRepository, get_image_repository },
        rendition::{ RenditionRepository, get_rendition_repository },
    },
    model::{
        image::Image, upload_image::UploadImage, rendition::Rendition,
    },
};

#[derive(Serialize)]
pub struct ImageResponse {
    images: Vec<Image>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageSaveResponse<'a> {
    success: bool,
    message: &'a str,
    image_id: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageTitleUpdateRequest {
    image_id: u32,
    title: String,
}

#[get("/api/admin/project/{project_id}/images")]
pub async fn get_images_in_project(req: HttpRequest) -> HttpResponse {
    let project_id: String = req.match_info().get("project_id")
        .unwrap().parse().unwrap();
    println!("Fetching images for project: {}", project_id);
    let repo = get_image_repository();
    let images_wrapped = repo.get_all_from_project(project_id.parse::<u32>().unwrap());


    match images_wrapped {
        Ok (images) => {
            HttpResponse::Ok().json(ImageResponse {images})
        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound()
                    .json(ImageResponse { images: vec![] });
            }

            eprintln!("Some internal error occured while fetching project images.");

            HttpResponse::InternalServerError()
                .json(ImageResponse { images: vec![] })
        }
    }
}

#[get("/api/admin/image/{image_id}")]
pub async fn get_image(req: HttpRequest) -> HttpResponse {
    let image_id:u32 = req.match_info().get("image_id").unwrap().parse()
        .unwrap();

    match get_image_repository().get(image_id) {
        Ok (image) => {
            println!("got id: {}, name: {}", image.id, image.name);
            HttpResponse::Ok().json(image)
        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound().body("Not Found");
            }

            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

#[post("/api/admin/image-save")]
pub async fn add_image(req_image: Json<UploadImage>) -> HttpResponse {
    println!("Got request for upload id: {}", req_image.upload_id);

    let mut image = Image {
        id: 0,
        name: req_image.name.clone(),
        title: req_image.title.clone(),
        encoding: req_image.encoding,
        height: 0,
        width: 0,
        is_published: true,
        project_id: req_image.project_id,
        folder_id: req_image.folder_id,
        created_on: Utc::now(),
        created_by: 0,
        modified_on: Utc::now(),
        modified_by: 0,
    };

    // Get image height and width from file
    let source_file_path: String = format!(
        "temp/{}{}",
        req_image.upload_id,
        image.encoding.extension()
    );

    println!("source file path: {}", source_file_path);

    let raster_img = raster::open(source_file_path.as_str()).unwrap();

    image.height = raster_img.height as u16;
    image.width = raster_img.width as u16;

    // Add image to the db
    match get_image_repository().add(image.clone()) {
        Ok (id) => {
            // Finally, change temp image path
            let dest_file_path = format!(
                "image-uploads/{}{}",
                id,                         // id of image after add transaction committed
                image.encoding.extension()
            );

            match rename(source_file_path, dest_file_path) {
                Ok (_) => HttpResponse::Ok().json(ImageSaveResponse {
                    success: true,
                    message: "Image Saved",
                    image_id: Some(id)
                }),

                Err (e) => {
                    eprintln!(
                        "An I/O error occured while adding an image: {}", e
                    );

                    return HttpResponse::InternalServerError().json(
                        ImageSaveResponse {
                            success: false,
                            message:
                                "There was some problem. Please try again.",
                            image_id: None
                    });
                }
            }
        }

        Err (_s) => HttpResponse::InternalServerError().json(
            ImageSaveResponse {
                success: false,
                message: "There was some problem. Please try again.",
                image_id: None
        })
    }
}

/**
 * Deletes the image data from the database and deletes the original image file
 * and rendition files.
 */
#[delete("/api/admin/image/{image_id}")]
pub async fn remove_image(req: HttpRequest) -> HttpResponse {
    let image_id:u32 = req.match_info().get("image_id").unwrap().parse()
        .unwrap();

    let mut renditions_exist = false;
    let renditions: Vec<Rendition>;
    let image: Image;

    // Get image object
    match get_image_repository().get(image_id) {
        Ok (img) => {
            image = img;
        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                eprintln!("Error in delete image api while getting image object: Image not found");
            } else {
                eprintln!("Unknown error in delete image api while getting image object!");
            }

            return HttpResponse::NotFound().body("Image not found!");
        }
    }

    // Get renditions
    match get_rendition_repository().get_all_from_image(image_id) {
        Ok(rens) => {
            if !rens.is_empty() {
                renditions_exist = true;
                renditions = rens;
            } else {
                renditions = vec![];
            }
        }

        Err(_) => {
            renditions = vec![];
        }
    }

    match get_image_repository().remove_item(image_id) {
        Ok (message) => {
            // Delete the image file if it exists
            match remove_file (
                format!(
                    "image-uploads/{}{}",
                    image_id,
                    image.encoding.extension()
            )) {
                Ok (_) => {}

                Err (e) => {
                    eprintln!("Error while deleting image file for image id: {}: {}", image_id, e);
                }
            }

            if renditions_exist {
                // Delete the rendition files from rendition cache if they exist.
                for rendition in renditions.iter() {
                    match remove_file (
                        format!(
                            "image-rendition-cache/{}{}",
                            rendition.id,
                            rendition.encoding.extension(),
                    )) {
                        Ok (_) => {}

                        Err (e) => {
                            eprintln!(
                                "Error while deleting rendition file (id: {}) for image id: {}: {}",
                                rendition.id,
                                image_id,
                                e
                            );
                        }
                    }
                }
            }

            HttpResponse::Ok().body(message)
        }

        Err (err_msg) => {
            HttpResponse::InternalServerError().body(err_msg)
        }
    }
}

/**
 * Gets the original image file for preview for admin user.
 */
#[get("/api/admin/image-file/{image_id}")]
pub async fn get_image_file(req: HttpRequest) -> HttpResponse {
    let image_id:u32 = req.match_info().get("image_id").unwrap().parse()
        .unwrap();

    let img_repo = get_image_repository();

    match img_repo.get(image_id) {
        Ok (image) => {
            let image_file_path = format!(
                "image-uploads/{}{}",
                image.id,
                image.encoding.extension()
            );

            let image_file = block(
                move || read(String::from(image_file_path))
            ).await.unwrap().expect("Error whie downloading!");

            HttpResponse::Ok()
                .content_type(image.encoding.mime_type())
                .body(image_file)
        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound().body("Image not found");
            }

            return HttpResponse::InternalServerError().body("Some error occured");
        }
    }
}

/**
 * Updates the title of an image.
 */
#[put("/api/admin/image/update-title/")]
pub async fn update_image_title (req: Json<ImageTitleUpdateRequest>)
    -> HttpResponse {
    let img_repo = get_image_repository();

    match img_repo.get(req.image_id) {
        Ok (mut image) => {
            image.title = req.title.clone();

            match img_repo.update(image) {
                Ok (msg) => HttpResponse::Ok().body(msg),

                Err (msg) => HttpResponse::InternalServerError().body(msg),
            }

        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound().body("Image not found");
            }

            return HttpResponse::InternalServerError().body("Some error occured");
        }
    }
}

