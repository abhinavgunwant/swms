use actix_web::{
    web::Json, HttpResponse, HttpRequest, post, get, http::header::ContentType,
};
use serde::Serialize;
use chrono::Utc;
use crate::{
    db::DBError,
    repository::image::{ ImageRepository, get_image_repository },
    model::{ image::Image, upload_image::UploadImage, encoding::Encoding },
};

#[derive(Serialize)]
pub struct ImageResponse {
    images: Vec<Image>
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

    let image = Image {
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

    // TODO: get image height and width
    // TODO: change temp image path

    if get_image_repository().add(image) {
        return HttpResponse::Ok().content_type(ContentType::json())
            .body("true");
    }

    HttpResponse::InternalServerError().content_type(ContentType::json())
        .body("false")
}

