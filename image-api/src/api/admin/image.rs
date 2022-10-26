use actix_web::{ web::{ Json }, HttpResponse, HttpRequest, cookie::Cookie, post, get };
use serde::{ Serialize, Deserialize };
use crate::db::DBError;
use crate::repository::{
    image::{
        Image, ImageRepository, get_image_repository
    },
    project::{
        Project, ProjectRepository, get_project_repository, validate_project
    },
    user::{ get_user_repository, User, UserRepository }
};

//#[derive(Deserialize)]
//pub struct ProjectImageRequest {
//    project_id: u32
//}

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
