use actix_web::{ HttpResponse, HttpRequest, get, post, web::Json };
use serde::{ Serialize, Deserialize };
use crate::{
    db::DBError,
    repository::{
        rendition::{ RenditionRepository, get_rendition_repository },
        image::{ ImageRepository, get_image_repository },
    },
    model::{ rendition::Rendition, image::Image },
};

#[derive(Serialize)]
pub struct RenditionResponse {
    renditions: Vec<Rendition>
}

#[derive(Deserialize)]
pub struct RenditionRequest {
    renditions: Vec<Rendition>,
    eager: bool,
}

#[get("/api/admin/renditions/{image_id}")]
pub async fn get_renditions_for_image(req: HttpRequest) -> HttpResponse {
    let image_id: String = String::from(req.match_info().get("image_id")
        .unwrap());
    let repo = get_rendition_repository();
    let renditions_wrapped = repo.get_all_from_image(image_id.parse::<u32>().unwrap());

    match renditions_wrapped {
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

#[post("/api/admin/renditions")]
pub async fn set_rendition(req: Json<RenditionRequest>) -> HttpResponse {
    let repo = get_rendition_repository();
    let img_repo = get_image_repository();

    let mut image: Option<Image> = None;

    for rendition in req.renditions.iter() {
        if req.eager {
            // TODO: Create renditions eagerly.
            println!("Creating renditions eagerly");

            if image.is_none() {
                // TODO: Initialize Image
                match img_repo.get(rendition.image_id) {
                    Ok(img) => {
                        image = Some(img);
                    }

                    Err (_e) => {
                        // TODO: Push "Error Image not found" if image does not exist
                    }
                }
            }

            let source_file_path = format!(
                "image-uploads/{}{}",
                image.as_ref().unwrap().id,
                image.as_ref().unwrap().encoding.extension()
            );

            let dest_file_path = format!(
                "image-rendition-cache/{}{}",
                rendition.id,
                rendition.encoding.extension()
            );

            let mut raster_img = raster::open(
                source_file_path.as_str()
            ).unwrap();

            raster::editor::resize(
                &mut raster_img,
                rendition.width as i32,
                rendition.height as i32,
                raster::ResizeMode::Fit
            ).unwrap();

            raster::save(&raster_img, dest_file_path.as_str()).unwrap();
        }

        repo.add(rendition.clone());
    }


    // TODO: implement error handling

    HttpResponse::Ok().body("Fix this")
}

