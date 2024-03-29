use std::rc::Rc;

use actix_web::{
    HttpResponse, HttpRequest, get, post, delete, web::{ Json, Data }
};
use serde::{ Serialize, Deserialize };
use raster::Image as RasterImage;
use qstring::QString;
use log::{ debug, error };

use crate::{
    api::service::{
        path::{ resize_and_save_rendition, get_image_path },
        remove::remove_rendition_file,
    },
    server::db::DBError, auth::AuthMiddleware, server::config::ServerConfig,
    repository::Repository, model::{ rendition::Rendition, image::Image },
};

#[derive(Serialize)]
pub struct StandardResponse<'a> {
    success: bool,
    message: &'a str,
}

#[derive(Serialize)]
pub struct RenditionResponse {
    renditions: Vec<Rendition>
}

#[derive(Deserialize)]
pub struct RenditionRequest {
    renditions: Vec<Rendition>,
    eager: bool,
}

#[derive(Serialize)]
pub struct SetRenditionsResponse {
    success: bool,
    message: String,
    // IDs of renditions that were not successfully saved.
    unsuccessful_renditions: Vec<UnsuccessfulRendition>,
}

#[derive(Serialize)]
pub struct UnsuccessfulRendition {
    id: u32,
    message: String,
}

/// Returns renditions for an image
///
/// ## URL Parameters
/// - `image-id` - (Required) The image id the renditions belong to.
#[get("/api/admin/renditions")]
pub async fn get_renditions_for_image(
    repo: Data<dyn Repository + Sync + Send>,
    req: HttpRequest,
    _: AuthMiddleware)
    -> HttpResponse {
    debug!("getting renditions");
    let qs = QString::from(req.query_string());

    let image_id: Option<u32>;


    if let Some(img_id_str) = qs.get("image-id") {
        match img_id_str.parse::<u32>() {
            Ok(id) => { image_id = Some(id); }
            Err(_) => { image_id = None; }
        }
    } else {
        image_id = None;
    }

    if image_id == None {
        return HttpResponse::BadRequest().body("BAD REQUEST");
    }

    match repo.get_rendition_repo() {
        Ok(mut ren_repo) => {
            match ren_repo.get_all_from_image(image_id.unwrap()) {
                Ok (renditions) => {
                    HttpResponse::Ok().json(RenditionResponse { renditions })
                }

                Err (e) => {
                    match e {
                        DBError::NotFound => HttpResponse::NotFound()
                            .json(RenditionResponse { renditions: vec![] }),
                        _ => {
                            error!("Some internal error occured while fetching \
                                project images.");

                            HttpResponse::InternalServerError()
                                .json(RenditionResponse { renditions: vec![] })
                        }
                    }
                }
            }
        }

        Err(e) => {
            error!("Error while getting rendition repository: {}", e);

            HttpResponse::InternalServerError()
                .json(RenditionResponse { renditions: vec![] })
        }
    }
}

#[get("/api/admin/rendition/{rendition_id}")]
pub async fn get_rendition(
    repo: Data<dyn Repository + Sync + Send>,
    req: HttpRequest,
    _: AuthMiddleware
) -> HttpResponse {
    let rendition_id: String = String::from(req.match_info().get("rendition_id")
        .unwrap());

    match repo.get_rendition_repo() {
        Ok(mut ren_repo) => {
            match ren_repo.get(rendition_id.parse::<u32>().unwrap()) {
                Ok (rendition) => {
                    HttpResponse::Ok().json(rendition)
                }

                Err (e) => {
                    match e {
                        DBError::NotFound => HttpResponse::NotFound()
                            .body("Not Found"),

                        _ => {
                            error!("Error while fetching rendition: {}", e);

                            HttpResponse::InternalServerError()
                                .body("Internal Server Error")
                        }
                    }
                }
            }
        }

        Err(e) => {
            error!("Error while getting rendition repository: {}", e);
            HttpResponse::InternalServerError()
                .body("Internal Server Error")
        }
    }
}

/// Creates multiple renditions for a single image.
#[post("/api/admin/renditions")]
pub async fn set_rendition(
    repository: Data<dyn Repository + Sync + Send>,
    req: Json<RenditionRequest>,
    _: AuthMiddleware,
    conf: Data<ServerConfig>
) -> HttpResponse {
    let mut unsuccessful_renditions: Vec<UnsuccessfulRendition> = vec![];
    let mut internal_error: bool = false;

    let image_option: Option<Image>;

    debug!("Inside add rendition API");

    if req.renditions.is_empty() {
        return HttpResponse::BadRequest()
            .json(SetRenditionsResponse {
                success: false,
                message: String::from("No renditions provided"),
                unsuccessful_renditions,
            });
    }

    let image_id = req.renditions[0].image_id;

    let different_images = req.renditions.iter()
        .filter(|&r| r.image_id != image_id)
        .count();

    if different_images > 0 {
        return HttpResponse::BadRequest().json(SetRenditionsResponse {
            success: false,
            message: String::from(
                "Renditions with different image_ids are not allowed!"
            ),
            unsuccessful_renditions,
        });
    }

    // Get image from repo.
    match repository.get_image_repo() {
        Ok(mut img_repo) => {
            match img_repo.get(image_id) {
                Ok(img) => { image_option = Some(img); },
                Err (e) => {
                    error!("Error while getting image data: {}", e);

                    return HttpResponse::BadRequest().json(SetRenditionsResponse {
                        success: false,
                        message: format!("Image having id \"{}\" not found", image_id),
                        unsuccessful_renditions,
                    });
                }
            }
        }

        Err(e) => {
            error!("Error while getting image repo: {}", e);

            return HttpResponse::InternalServerError()
                .body("Some internal error occured!");
        }
    }

    if let Some(image) = image_option {
        let mut image_raster_option: Option<Rc<RasterImage>> = None;
        let image_path;

        match get_image_path(&repository, &image) {
            Ok(i_path) => { image_path = i_path; }
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .json(SetRenditionsResponse {
                    success: false,
                    message: String::from("No renditions saved!"),
                    unsuccessful_renditions,
                });
            }
        }

        debug!("Got the image path: {}", image_path);

        let dest_path_prefix = format!(
            "{}/{}", conf.rendition_cache_dir, image_path
        );

        if req.eager {
            let src_file_path = format!(
                "{}/{}{}", conf.upload_dir, image.id, image.encoding.extension()
            );

            match raster::open(src_file_path.as_str()) {
                Ok (r_img) => { image_raster_option = Some(Rc::new(r_img)); }
                Err (_) => {
                    error!("Error loading image file: {}", src_file_path);
                }
            }
        }

        for rendition in req.renditions.iter() {
            let mut rendition_to_add: Rendition = rendition.clone();

            if rendition.slug == "default" {
                rendition_to_add.height = image.height;
                rendition_to_add.width = image.width;
            }

            match repository.get_rendition_repo() {
                Ok(mut ren_repo) => {
                    match ren_repo.add(rendition_to_add.clone()) {
                        Ok (_rendition_id) => {
                            // Create renditions files.
                            if let Some(mut r_img) = image_raster_option.clone() {
                                let dest_file_path = format!(
                                    "{}/{}{}",
                                    dest_path_prefix,
                                    rendition_to_add.slug,
                                    rendition_to_add.encoding.extension()
                                );

                                let dest_path = dest_file_path.as_str();

                                debug!(
                                    "--> Creating rendition eagerly: {} ({}x{})",
                                    dest_file_path ,
                                    rendition_to_add.width,
                                    rendition_to_add.height,
                                );

                                match resize_and_save_rendition(
                                    Rc::make_mut(&mut r_img),
                                    dest_path,
                                    rendition_to_add.width,
                                    rendition_to_add.height
                                ) {
                                    Ok(_) => {},
                                    Err(_) => {},
                                };
                            }
                        }

                        Err (err_msg) => {
                            internal_error = true;

                            unsuccessful_renditions.push(UnsuccessfulRendition {
                                id: rendition.id,
                                message: err_msg,
                            });
                        }
                    }
                }

                Err(e) => {
                    error!("Error while getting rendition repository: {}", e);

                    internal_error = true;

                    unsuccessful_renditions.push(UnsuccessfulRendition {
                        id: rendition.id,
                        message: String::from("Internal server error."),
                    });
                }
            }
        }
    }

    if unsuccessful_renditions.len() > 0 {
        if unsuccessful_renditions.len() == req.renditions.len() {
            if internal_error {
                return HttpResponse::InternalServerError()
                    .json(SetRenditionsResponse {
                    success: false,
                    message: String::from("No renditions saved!"),
                    unsuccessful_renditions,
                });
            }

            return HttpResponse::BadRequest().json(SetRenditionsResponse {
                success: false,
                message: String::from("No renditions saved!"),
                unsuccessful_renditions,
            });
        }

        return HttpResponse::BadRequest().json(SetRenditionsResponse {
            success: false,
            message: String::from("Some or all renditions not saved!"),
            unsuccessful_renditions,
        });
    }

    HttpResponse::Ok().json(SetRenditionsResponse {
        success: true,
        message: String::from("All renditions saved successfully"),
        unsuccessful_renditions,
    })
}

/// Creates multiple renditions for a single image.
#[delete("/api/admin/rendition/{rendition_id}")]
pub async fn delete_rendition(
    repository: Data<dyn Repository + Sync + Send>,
    req: HttpRequest,
    _: AuthMiddleware,
    conf: Data<ServerConfig>
) -> HttpResponse {
    if let Some(rid) = req.match_info().get("rendition_id") {
        let mut ren_repo;

        match repository.get_rendition_repo() {
            Ok(r_repo) => { ren_repo = r_repo; }
            Err(e) => {
                let msg = "Error while getting rendition repo";
                error!("{}: {}", msg, e);

                return HttpResponse::InternalServerError()
                    .json(StandardResponse {
                    success: false,
                    message: "Some internal error occured!",
                });
            }
        }

        if let Ok(rendition_id) = rid.parse::<u32>() {
            if let Ok(rendition) = ren_repo.get(rendition_id) {
                match ren_repo.remove_item(rendition_id) {
                    Ok(_) => {
                        let mut image_path: String = String::default();

                        match repository.get_image_repo() {
                            Ok(mut img_repo) => {
                                if let Ok(image) = img_repo.get(
                                    rendition.image_id
                                ) {
                                    if let Ok(img_path) = get_image_path(
                                        &repository, &image
                                    ) {
                                        image_path = img_path;
                                    }
                                }
                            }

                            Err(e) => {
                                error!(
                                    "Error while getting image repo: {}", e
                                );
                            }
                        }

                        if image_path.is_empty() {
                            return HttpResponse::Ok().json(StandardResponse {
                                success: true,
                                message: "Rendition deleted but rendition cache \
                                    file could not be deleted. The cache file \
                                    could not be found.",
                            });
                        }

                        if remove_rendition_file(
                            &conf.rendition_cache_dir, &image_path, &rendition
                        ) {
                            return HttpResponse::Ok().json(StandardResponse {
                                success: true,
                                message: "Rendition deleted",
                            });
                        } else {
                            return HttpResponse::InternalServerError()
                                .json(StandardResponse {
                                success: true,
                                message: "Rendition deleted but error in deleting \
                                    the rendition cache.",
                            });
                        }
                    }

                    Err (_e) => {
                        return HttpResponse::InternalServerError().json(StandardResponse {
                            success: false,
                            message: "Some error occured while deleting rendition",
                        });
                    }
                }
            }

            return HttpResponse::NotFound().json(StandardResponse {
                success: false,
                message: "Supplied rendition was not found.",
            });
        }
    }

    HttpResponse::BadRequest().body("BAD REQUEST")
}

