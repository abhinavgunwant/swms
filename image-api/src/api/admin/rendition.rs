use actix_web::{ HttpResponse, HttpRequest, get, post, delete, web::Json };
use serde::{ Serialize, Deserialize };
use raster::Image as RasterImage;
use qstring::QString;

use crate::{
    db::DBError,
    repository::{
        rendition::{ RenditionRepository, get_rendition_repository },
        image::{ ImageRepository, get_image_repository },
    },
    model::{ rendition::Rendition, image::Image },
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
pub async fn get_renditions_for_image(req: HttpRequest) -> HttpResponse {
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

    let repo = get_rendition_repository();

    match repo.get_all_from_image(image_id.unwrap()) {
        Ok (renditions) => {
            HttpResponse::Ok().json(RenditionResponse { renditions })
        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound()
                    .json(RenditionResponse { renditions: vec![] });
            }

            eprintln!("Some internal error occured while fetching project images.");

            HttpResponse::InternalServerError()
                .json(RenditionResponse { renditions: vec![] })
        }
    }
}

#[get("/api/admin/rendition/{rendition_id}")]
pub async fn get_rendition(req: HttpRequest) -> HttpResponse {
    let rendition_id: String = String::from(req.match_info().get("rendition_id")
        .unwrap());
    let repo = get_rendition_repository();
    let rendition_wrapped = repo.get(rendition_id.parse::<u32>().unwrap());

    match rendition_wrapped {
        Ok (rendition) => {
            HttpResponse::Ok().json(rendition)
        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound().body("Not Found");
            }

            eprintln!("Some internal error occured while fetching rendition.");

            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

/**
 * Creates multiple renditions for a single image.
 */
#[post("/api/admin/renditions")]
pub async fn set_rendition(req: Json<RenditionRequest>) -> HttpResponse {
    let mut success: bool = true;
    let mut unsuccessful_renditions: Vec<UnsuccessfulRendition> = vec![];
    let mut internal_error: bool = false;

    let mut raster_img = RasterImage{ width: 0, height: 0, bytes: vec![] };
    let mut raster_img_init: bool = false;

    let mut source_file_path: String = String::from("");
    let mut source_file_path_init: bool = false;


    let repo = get_rendition_repository();
    let img_repo = get_image_repository();

    let mut image: Option<Image> = None;

    for rendition in req.renditions.iter() {
        if req.eager {
            println!("Creating renditions eagerly");

            if image.is_none() {
                match img_repo.get(rendition.image_id) {
                    Ok(img) => {
                        image = Some(img);
                    }

                    Err (_e) => {
                        unsuccessful_renditions.push(UnsuccessfulRendition {
                            id: rendition.id,
                            message: format!(
                                "Image having id \"{}\" not found",
                                rendition.image_id
                            ),
                        });

                        success = false;
                    }
                }
            }

            match repo.add(rendition.clone()) {
                Ok (rendition_id) => {
                    if !source_file_path_init {
                        source_file_path = format!(
                            "image-uploads/{}{}",
                            image.as_ref().unwrap().id,
                            image.as_ref().unwrap().encoding.extension()
                        );

                        source_file_path_init = true;
                    }

                    let dest_file_path = format!(
                        "image-rendition-cache/{}{}",
                        rendition_id,
                        rendition.encoding.extension()
                    );

                    if !raster_img_init {
                        raster_img = raster::open(
                            source_file_path.as_str()
                        ).unwrap();

                        raster_img_init = true;
                    }

                    raster::editor::resize(
                        &mut raster_img,
                        rendition.width as i32,
                        rendition.height as i32,
                        raster::ResizeMode::Fit
                    ).unwrap();

                    raster::save(&raster_img, dest_file_path.as_str()).unwrap();
                }

                Err (err_msg) => {
                    internal_error = true;

                    unsuccessful_renditions.push(UnsuccessfulRendition {
                        id: rendition.id,
                        message: err_msg,
                    });
                }
            }
        } else {
            match repo.add(rendition.clone()) {
                Ok (_id) => {},
                Err (err_msg) => {
                    internal_error = true;

                    unsuccessful_renditions.push(UnsuccessfulRendition {
                        id: rendition.id,
                        message: err_msg,
                    });
                }
            }
        }
    }

    if !success || unsuccessful_renditions.len() > 0 {
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
        success,
        message: String::from("All renditions saved successfully"),
        unsuccessful_renditions,
    })
}

/// Creates multiple renditions for a single image.
#[delete("/api/admin/rendition/{rendition_id}")]
pub async fn delete_rendition(req: HttpRequest) -> HttpResponse {
    if let Some(rid) = req.match_info().get("rendition_id") {
        let repo = get_rendition_repository();

        if let Ok(rendition_id) = rid.parse::<u32>() {
            match repo.remove_item(rendition_id) {
                Ok (_) => {
                    return HttpResponse::Ok().json(StandardResponse {
                        success: true,
                        message: "Rendition deleted",
                    });
                }

                Err (_e) => {
                    return HttpResponse::InternalServerError().json(StandardResponse {
                        success: false,
                        message: "Some error occured while deleting rendition",
                    });
                }
            }
        }
    }

    HttpResponse::BadRequest().body("BAD REQUEST")
}

