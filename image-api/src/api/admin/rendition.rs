use std::rc::Rc;

use actix_web::{ HttpResponse, HttpRequest, get, post, delete, web::Json };
use serde::{ Serialize, Deserialize };
use raster::Image as RasterImage;
use qstring::QString;

use crate::{
    api::{
        DEST_REN_DIR, IMG_UPL_DIR,
        service::path::{ resize_and_save_rendition, get_image_path }
    },
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

/// Creates multiple renditions for a single image.
#[post("/api/admin/renditions")]
pub async fn set_rendition(req: Json<RenditionRequest>) -> HttpResponse {
    let mut unsuccessful_renditions: Vec<UnsuccessfulRendition> = vec![];
    let mut internal_error: bool = false;

    let repo = get_rendition_repository();
    let img_repo = get_image_repository();

    let image_option: Option<Image>;

    println!("Inside add rendition API");

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
    match img_repo.get(image_id) {
        Ok(img) => { image_option = Some(img); },
        Err (e) => {
            eprintln!("{}", e);

            return HttpResponse::BadRequest().json(SetRenditionsResponse {
                success: false,
                message: format!("Image having id \"{}\" not found", image_id),
                unsuccessful_renditions,
            });
        }
    }

    if let Some(image) = image_option {
        let mut image_raster_option: Option<Rc<RasterImage>> = None;
        let image_path;

        match get_image_path(image.clone()) {
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

        println!("Got the image path: {}", image_path);

        let dest_path_prefix = format!("{}/{}", DEST_REN_DIR, image_path);

        if req.eager {
            let src_file_path = format!(
                "image-uploads/{}{}", image.id, image.encoding.extension()
            );

            match raster::open(src_file_path.as_str()) {
                Ok (r_img) => { image_raster_option = Some(Rc::new(r_img)); }
                Err (_) => { eprintln!("Error loading image file."); }
            }
        }

        for rendition in req.renditions.iter() {
            let mut rendition_to_add: Rendition = rendition.clone();

            if rendition.slug == "default" {
                rendition_to_add.height = image.height;
                rendition_to_add.width = image.width;
            }

            match repo.add(rendition_to_add.clone()) {
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

                        println!(
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

