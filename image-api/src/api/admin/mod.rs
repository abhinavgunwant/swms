pub mod image;
pub mod auth;
pub mod user;
pub mod project;

use actix_web::{
    web::{ Json }, HttpResponse, HttpRequest, cookie::Cookie, post, get
};
use serde::{ Serialize, Deserialize };
use qstring::QString;
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

#[derive(Deserialize)]
pub struct GetChildrenRequest {
    #[serde(rename = "type")]
    _type: String,
    slug: String,
}

#[derive(Serialize)]
pub struct GetChildrenResponse {
    folders: Vec<u32>, // TODO: replace this with vector of `Folder`
    images: Vec<Image>,
    success: bool,
    message: Vec<String>,
}

#[get("/api/admin/get-children")]
pub async fn get_children(req: HttpRequest) -> HttpResponse {
    let qs = QString::from(req.query_string());

    let _type = qs.get("type").unwrap();
    let slug = String::from(qs.get("slug").unwrap());

    let repo = get_image_repository();
    let images_wrapped = repo.get_all_from_project_slug(slug);

    let mut response_images: Vec<Image> = vec![];
    let mut response_folders:Vec<u32> = vec![];
    let mut response_msg: Vec<String> = vec![];

    let mut images_found: bool = false;
    let mut folders_found: bool = false;
    let mut error: bool = false;

    // TODO: Check _type here to decide which repo to pull children from

    // collect images
    match images_wrapped {
        Ok (images) => {
            //HttpResponse::Ok().json(ImageResponse {images})
            response_images = images;
            images_found = true;
        }

        Err (e) => {
            eprintln!("Some internal error occured while fetching project images.");

            error = true;
        }
    }

    // collect folders
    // TODO: fetch folders under the current project/folder

    folders_found = true; // TODO: modify based on whether folder fetched or not.

    if !images_found {
        response_msg.push(String::from("IMAGES NOT FOUND"));
    }

    if !folders_found {
        response_msg.push(String::from("FOLDERS NOT FOUND"));
    }
    
    if images_found && folders_found {
        response_msg.push(String::from("SUCCESS"));
    }

    if error {
        return HttpResponse::InternalServerError().json(GetChildrenResponse {
            images: response_images,
            folders: response_folders,
            success: images_found || folders_found,
            message: response_msg,
        });
    }
    
    if images_found || folders_found {
        return HttpResponse::Ok().json(GetChildrenResponse {
            images: response_images,
            folders: response_folders,
            success: images_found || folders_found,
            message: response_msg,
        });
    }

    HttpResponse::NotFound().json(GetChildrenResponse {
        images: response_images,
        folders: response_folders,
        success: images_found || folders_found,
        message: response_msg,
    })
}
