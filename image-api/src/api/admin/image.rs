use actix_web::{
    web::{ Json, block, Data }, HttpResponse, HttpRequest, post, get, put, delete,
};
use std::fs::{ read, rename };
use serde::{ Serialize, Deserialize };
use qstring::QString;
use chrono::Utc;
use log::{ debug, error };

use crate::{
    db::DBError, auth::AuthMiddleware, server::config::ServerConfig,
    repository::Repository,
    model::{ image::Image, upload_image::UploadImage },
    api::{ admin::SuccessResponse, service::remove::remove_images, },
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
pub async fn get_images_in_project(
    repo: Data<dyn Repository + Sync + Send>,
    req: HttpRequest,
    _: AuthMiddleware
) -> HttpResponse {
    let project_id: String = req.match_info().get("project_id")
        .unwrap().parse().unwrap();
    debug!("Fetching images for project: {}", project_id);
    let images_wrapped;

    match repo.get_image_repo() {
        Ok(img_repo) => {
            images_wrapped = img_repo.get_all_from_project(
                project_id.parse::<u32>().unwrap()
            );
        }

        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("Some internal error occured!");
        }
    }


    match images_wrapped {
        Ok (images) => {
            HttpResponse::Ok().json(ImageResponse {images})
        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound()
                    .json(ImageResponse { images: vec![] });
            }

            error!("Some internal error occured while fetching project images.");

            HttpResponse::InternalServerError()
                .json(ImageResponse { images: vec![] })
        }
    }
}

#[get("/api/admin/image/{image_id}")]
pub async fn get_image(
    repo: Data<dyn Repository + Sync + Send>,
    req: HttpRequest,
    _: AuthMiddleware
) -> HttpResponse {
    let image_id:u32 = req.match_info().get("image_id").unwrap().parse()
        .unwrap();

    match repo.get_image_repo() {
        Ok(img_repo) => {
            match img_repo.get(image_id) {
                Ok (image) => {
                    debug!("got id: {}, name: {}", image.id, image.name);
                    HttpResponse::Ok().json(image)
                }

                Err (e) => {
                    if e == DBError::NOT_FOUND {
                        return HttpResponse::NotFound().body("Not Found");
                    }

                    HttpResponse::InternalServerError()
                        .body("Internal Server Error")
                }
            }
        }

        Err(_) => HttpResponse::InternalServerError()
            .body("Internal Server Error")
    }
}

#[post("/api/admin/image-save")]
pub async fn add_image(
    repo: Data<dyn Repository + Sync + Send>,
    req_image: Json<UploadImage>,
    _: AuthMiddleware,
    conf: Data<ServerConfig>
) -> HttpResponse {
    debug!("Got request for upload id: {}", req_image.upload_id);

    let mut image = Image {
        id: 0,
        name: req_image.name.clone(),
        title: req_image.title.clone(),
        slug: req_image.slug.clone(),
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

    debug!("source file path: {}", source_file_path);

    let raster_img = raster::open(source_file_path.as_str()).unwrap();

    image.height = raster_img.height as u16;
    image.width = raster_img.width as u16;

    // Add image to the db
    match repo.get_image_repo() {
        Ok(img_repo) => {
            match img_repo.add(image.clone()) {
                Ok (id) => {
                    // Finally, change temp image path
                    let dest_file_path = format!(
                        "{}/{}{}",
                        conf.upload_dir,
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
                            error!(
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

        Err(_) => HttpResponse::InternalServerError().json(ImageSaveResponse {
            success: false,
            message: "There was some problem. Please try again.",
            image_id: None
        })
    }
}

/// Deletes the image data from the database and deletes the original image file
/// and rendition files.
///
/// ## URL parameters:
/// - `id` - Comma-separated image IDs.
#[delete("/api/admin/image")]
pub async fn remove_image(
    repo: Data<dyn Repository + Sync + Send>, req: HttpRequest, _: AuthMiddleware,
    conf: Data<ServerConfig>,
) -> HttpResponse {
    let qs = QString::from(req.query_string());

    let image_ids: Vec<u32>;

    match qs.get("id") {
        Some (qid) => {
            image_ids = qid.split(',').map(|s| s.parse().unwrap()).collect();
        }

        None => {
            return HttpResponse::BadRequest().body("No image IDs supplied");
        }
    }

    match remove_images(
        &repo, &image_ids, conf.rendition_cache_dir.clone(), conf.upload_dir.clone()
    ) {
        Ok (_) => {
            if image_ids.len() > 1 {
                return HttpResponse::Ok().body("Images deleted successfully");
            } else {
                return HttpResponse::Ok().body("Image deleted successfully");
            }
        }

        Err (_) => {
            if image_ids.len() > 1 {
                return HttpResponse::InternalServerError()
                    .body("Some images could not be deleted successfully");
            } else {
                return HttpResponse::InternalServerError()
                    .body("An error occurred while deleting image.");
            }
        }
    }
}

/**
 * Gets the original image file for preview for admin user.
 */
#[get("/api/admin/image-file/{image_id}")]
pub async fn get_image_file(
    repo: Data<dyn Repository + Sync + Send>,
    req: HttpRequest,
    _: AuthMiddleware,
    conf: Data<ServerConfig>
) -> HttpResponse {
    let image_id:u32 = req.match_info().get("image_id").unwrap().parse()
        .unwrap();

    match repo.get_image_repo() {
        Ok(img_repo) => {
            match img_repo.get(image_id) {
                Ok (image) => {
                    let image_file_path = format!(
                        "{}/{}{}",
                        conf.upload_dir,
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
                        return HttpResponse::NotFound()
                            .body("Image not found");
                    }

                    HttpResponse::InternalServerError()
                        .body("Some error occured")
                }
            }
        }

        Err(_) => HttpResponse::InternalServerError().body("Some error occured")
    }
}

/// Updates an image.
#[put("/api/admin/image")]
pub async fn update(
    repo: Data<dyn Repository + Sync + Send>,
    req: Json<Image>,
    _: AuthMiddleware
) -> HttpResponse {
    match repo.get_image_repo() {
        Ok(img_repo) => {
            match img_repo.update(req.into_inner()) {
                Ok (msg) => HttpResponse::Ok()
                    .json(SuccessResponse::new(true, msg)),
                Err (msg) => HttpResponse::InternalServerError()
                    .json(SuccessResponse::new(false, msg)),
            }
        }

        Err(_) => HttpResponse::InternalServerError()
                .json(SuccessResponse::new(
                    false,
                    String::from("Some internal server error occurred."),
                ))
    }
}

