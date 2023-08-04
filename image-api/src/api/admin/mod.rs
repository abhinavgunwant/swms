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
        project::{ ProjectRepository, get_project_repository },
        rendition::{ RenditionRepository, get_rendition_repository },
    },
    model::{ image::Image, folder::Folder, rendition::Rendition },
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
    rendition: Option<Rendition>,
    success: bool,
    message: Vec<String>,
}

#[derive(PartialEq)]
pub enum ResourceType {
    Project,
    Folder,
    Rendition,
    NONE,
}

/**
 * Returns the children of a project or a folder.
 * 
 * URL parameters:
 * - `type` - Optional. The type of resource.
 * - `path` - Required. The path of resource.
 *
 * e.g.:
 * /api/admin/get-children?type=<type>&path=<path>
 */
#[get("/api/admin/get-children")]
pub async fn get_children(req: HttpRequest) -> HttpResponse {
    let qs = QString::from(req.query_string());

    let mut response_msg: Vec<String> = vec![];

    let mut images_found: bool = false;
    let mut folders_found: bool = false;
    let mut error: bool = false;

    // URL parameter vars:
    let _type: ResourceType;
    let path: &str;

    // Check if "type" parameter is supplied with the request
    if let Some(t) = qs.get("type") {
        match t.to_uppercase().clone().as_str() {
            "PROJECT" => { _type = ResourceType::Project }
            "FOLDER" => { _type = ResourceType::Folder }
            "RENDITION" => { _type = ResourceType::Rendition }
            "" => {
                _type = ResourceType::NONE;
            }

            _ => {
                _type = ResourceType::NONE;
                error = true;

                response_msg.push(String::from(
                    "ERROR: Invalid \"type\" parameter."
                ));
            }
        }
    } else {
        _type = ResourceType::NONE;
    }

    match qs.get("path") {
        Some(s) => {
            let mut chars = s.chars();
            let mut updated = false;

            //if s.chars().last().unwrap() == '/' {
            if chars.clone().last().unwrap() == '/' {
                chars.next_back();
                updated = true;
            }

            let mut p = chars.clone().peekable();

            if let Some(first_char) = p.peek() {
                if *first_char == '/' {
                    chars.next();
                    updated = true;
                }
            }

            if updated {
                path = chars.as_str();
            } else {
                path = s;
            }
        }

        None => {
            path = "";
            error = true;

            response_msg.push(String::from(
                "ERROR: Request missing a \"path\" parameter."
            ));
        }
    }

    // Any errors discovered before this point are due to bad requests.
    if error {
        return HttpResponse::BadRequest().json(GetChildrenResponse {
            images: vec![],
            folders: vec![],
            success: false,
            rendition: None,
            message: response_msg,
        });
    }

    let img_repo = get_image_repository();
    let ren_repo = get_rendition_repository();
    let fol_repo = get_folder_repository();
    let proj_repo = get_project_repository();

    println!("Path is: {}", path);

    let path_segments: Vec<&str> = path.split("/").collect();
    let project_name: String;

    println!("Validating project slug: {}", path_segments[0]);

    // Validate project
    match proj_repo.validate_project_slug(path_segments[0].to_owned()) {
        Ok (valid) => {
            if !valid {
                error = true;
                project_name = String::from("");
                response_msg.push(String::from("Invalid project!"));
            } else {
                println!("\t-> Project Valid!");
                project_name = String::from(path_segments[0]);
            }
        }

        Err (err) => {
            error = true;
            project_name = String::from("");
            response_msg.push(format!(
                "Some error occured while getting project {}", err
            ));

            eprintln!("Error: {}", err);
        }
    }

    let mut images_wrapped: Result<Vec<Image>, DBError> = Err(DBError::NOT_FOUND);
    let mut response_images: Vec<Image> = vec![];
    let mut folders_wrapped: Result<Vec<Folder>, DBError> = Err(DBError::NOT_FOUND);
    let mut response_folders: Vec<Folder> = vec![];

    // if only project slug is supplied, return project object, otherwise
    // iterate and verify until last path_segment and return the last object.
    if path_segments.len() == 1
        && (_type == ResourceType::Project || _type == ResourceType::NONE)
    {
        images_wrapped = img_repo.get_all_from_project_slug(String::from(path));
        folders_wrapped = fol_repo.get_all_from_project_slug(String::from(path));
    } else {
        let path_seg_len = path_segments.len();

        for (i, path_segment) in path_segments[1..path_segments.len()].into_iter().enumerate() {
            println!("i : {}", i);
            // The last slug is usually the image rendition slug.
            if i == path_seg_len - 2
                && (
                    _type == ResourceType::Rendition
                    || _type == ResourceType::NONE
                )
            {
                println!("Validating image slug: {}", path_segment);
                match ren_repo.get_from_project_rendition_slug(
                    project_name.clone(),
                    String::from(*path_segment)
                ) {
                    Ok (rendition) => {
                        println!("\t-> Returning Image!");

                        // TODO: Check if the user has access.
                        response_msg.push(String::from("RENDITION"));

                        return HttpResponse::Ok().json(GetChildrenResponse {
                            images: response_images,
                            folders: response_folders,
                            success: true,
                            rendition: Some(rendition),
                            message: response_msg,
                        });
                    }

                    Err (_) => {}
                }
            }

            let path_seg_owned = String::from(*path_segment);
            println!("Validating folder slug: {}", path_segment);

            folders_wrapped = fol_repo.get_all_from_folder_slug(path_seg_owned.clone());
            images_wrapped = img_repo.get_all_from_folder_slug(path_seg_owned.clone());

//            match fol_repo.get_from_slug(path_seg_owned.clone()) {
//                Ok (folder) => {
//                    println!("\t-> Folder Valid!");
//
//                    // TODO: Check if the user has access.
//                    response_msg.push(String::from("FOLDER"));
//
//                    if i == path_seg_len - 2
//                        && (
//                            _type == ResourceType::Folder
//                            || _type == ResourceType::NONE
//                        )
//                    {
//                        println!("\t-> Returning Folder!");
//
//                        return HttpResponse::Ok().json(GetChildrenResponse {
//                            images: vec![],
//                            folders: vec![folder],
//                            success: true,
//                            rendition: None,
//                            message: response_msg,
//                        });
//                    }
//                }
//
//                Err (e) => {
//                    if e == DBError::NOT_FOUND {
//                        response_msg.push(String::from("Not found"));
//
//                        return HttpResponse::NotFound().json(GetChildrenResponse {
//                            images: vec![],
//                            folders: vec![],
//                            success: false,
//                            rendition: None,
//                            message: response_msg,
//                        });
//                    }
//
//                    response_msg.push(String::from("Some unknown error encountered."));
//
//                    return HttpResponse::InternalServerError().json(GetChildrenResponse {
//                        images: vec![],
//                        folders: vec![],
//                        success: false,
//                        rendition: None,
//                        message: response_msg,
//                    });
//                }
//            }
        }
    }

    // collect images
    match images_wrapped {
        Ok (images) => {
            println!("Found wrapped images.");
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
            println!("Found wrapped folders.");
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
    } else {
        response_msg.push(String::from("SUCCESS"));
    }

    if error {
        return HttpResponse::InternalServerError().json(GetChildrenResponse {
            images: response_images,
            folders: response_folders,
            success: images_found || folders_found,
            rendition: None,
            message: response_msg,
        });
    }
    
    if images_found || folders_found {
        return HttpResponse::Ok().json(GetChildrenResponse {
            images: response_images,
            folders: response_folders,
            success: images_found || folders_found,
            rendition: None,
            message: response_msg,
        });
    }

    HttpResponse::NotFound().json(GetChildrenResponse {
        images: response_images,
        folders: response_folders,
        success: images_found || folders_found,
        rendition: None,
        message: response_msg,
    })
}

