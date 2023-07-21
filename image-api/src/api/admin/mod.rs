pub mod image;
pub mod auth;
pub mod user;
pub mod project;
pub mod rendition;
pub mod role;
pub mod folder;

use actix_web::{ HttpResponse, HttpRequest, get };
use serde::{ Serialize, Deserialize };
use qstring::QString;

use crate::{
    db::DBError,
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
 * /api/admin/get-children?type=<type>&slug=<slug>
 */
#[get("/api/admin/get-children")]
pub async fn get_children(req: HttpRequest) -> HttpResponse {
    let qs = QString::from(req.query_string());

    let mut response_msg: Vec<String> = vec![];

    let mut images_found: bool = false;
    let mut folders_found: bool = false;
    let mut error: bool = false;

    let _type: &str;
    let slug: &str;

    // Check if "type" parameter is supplied with the request, if not, return
    // error.
    match qs.get("type") {
        Some(t) => {
            match t.to_uppercase().as_str() {
                "PROJECT" | "FOLDER" => { _type = t; }

                _ => {
                    _type = "";
                    error = true;

                    response_msg.push(String::from(
                        "ERROR: Invalid \"type\" parameter."
                    ));
                }
            }
        }

        None => {
            _type = "";
            error = true;

            response_msg.push(String::from(
                "ERROR: Request missing a \"type\" parameter."
            ));
        }
    }

    match qs.get("slug") {
        Some(s) => { slug = s; }

        None => {
            slug = "";
            error = true;

            response_msg.push(String::from(
                "ERROR: Request missing a \"slug\" parameter."
            ));
        }
    }

    // Any errors discovered before this point are due to bad requests.
    if error {
        return HttpResponse::BadRequest().json(GetChildrenResponse {
            images: vec![],
            folders: vec![],
            success: false,
            message: response_msg,
        })
    }

    let img_repo = get_image_repository();
    let fol_repo = get_folder_repository();

    let images_wrapped: Result<Vec<Image>, DBError>;
    let mut response_images: Vec<Image> = vec![];
    let folders_wrapped: Result<Vec<Folder>, DBError>;
    let mut response_folders: Vec<Folder> = vec![];

    match _type.to_uppercase().as_str() {
        "FOLDER" => {
            images_wrapped = img_repo.get_all_from_folder_slug(String::from(slug));
            folders_wrapped = fol_repo.get_all_from_folder_slug(String::from(slug));
        }

        "PROJECT" | _ => {
            images_wrapped = img_repo.get_all_from_project_slug(String::from(slug));
            folders_wrapped = fol_repo.get_all_from_project_slug(String::from(slug));
        }
    }

    // collect images
    match images_wrapped {
        Ok (images) => {
            response_images = images;

            if !response_images.is_empty() {
                images_found = true;
            }
        }

        Err (e) => {
            eprintln!("Some internal error occured while fetching project images: {}", e);

            response_msg.push(String::from(
                "Some internal error occured while fetching images."
            ));

            error = true;
        }
    }

    // collect folders
    match folders_wrapped {
        Ok (folders) => {
            response_folders = folders;

            if !response_folders.is_empty() {
                folders_found = true;
            }
        }

        Err (e) => {
            eprintln!("Some internal error occured while fetching project folders: {}", e);

            response_msg.push(String::from(
                "Some internal error occured while fetching folders."
            ));

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

