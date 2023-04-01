pub mod image;
pub mod auth;
pub mod user;
pub mod project;
pub mod rendition;
pub mod role;

use actix_web::{ HttpResponse, HttpRequest, get };
use serde::{ Serialize, Deserialize };
use qstring::QString;

use crate::{
    repository::{
        image::{ ImageRepository, get_image_repository },
        folder::{ FolderRepository, get_folder_repository },
    },
    model::{ image::Image, folder::Folder },
};

#[derive(Deserialize)]
pub struct GetChildrenRequest {
    #[serde(rename = "type")]
    _type: String,
    slug: String,
}

#[derive(Serialize)]
pub struct GetChildrenResponse {
    folders: Vec<Folder>, // TODO: replace this with vector of `Folder`
    images: Vec<Image>,
    success: bool,
    message: Vec<String>,
}

/**
 * Returns the children of a project or a folder.
 * 
 * e.g.:
 *
 * /api/admin/get-children?folder=<folder-slug>
 */
#[get("/api/admin/get-children")]
pub async fn get_children(req: HttpRequest) -> HttpResponse {
    let qs = QString::from(req.query_string());

    let _type = qs.get("type").unwrap();
    let slug = String::from(qs.get("slug").unwrap());

    let img_repo = get_image_repository();
    let fol_repo = get_folder_repository();
    let images_wrapped = img_repo.get_all_from_project_slug(slug.clone());
    let folders_wrapped = fol_repo.get_all_from_project_slug(slug);

    let mut response_images: Vec<Image> = vec![];
    let mut response_folders: Vec<Folder> = vec![];
    let mut response_msg: Vec<String> = vec![];

    let mut images_found: bool = false;
    let mut folders_found: bool = false;
    let mut error: bool = false;

    // TODO: Check _type here to decide which repo to pull children from

    // collect images
    match images_wrapped {
        Ok (images) => {
            response_images = images;
            images_found = true;
        }

        Err (_e) => {
            eprintln!("Some internal error occured while fetching project images.");

            error = true;
        }
    }

    // collect folders
    match folders_wrapped {
        Ok (folders) => {
            response_folders = folders;
            folders_found = true;
        }

        Err (_e) => {
            eprintln!("Some internal error occured while fetching project folders.");

            error = true;
        }
    }

    if !(images_found || folders_found) {
        response_msg.push(String::from("Found no content!"));
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

