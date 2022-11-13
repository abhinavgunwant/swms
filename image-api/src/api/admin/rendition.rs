use actix_web::{ HttpResponse, HttpRequest, get };
use serde::Serialize;
use crate::{
    db::DBError,
    repository::rendition::{ RenditionRepository, get_rendition_repository },
    model::rendition::Rendition,
};

#[derive(Serialize)]
pub struct RenditionResponse {
    renditions: Vec<Rendition>
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
