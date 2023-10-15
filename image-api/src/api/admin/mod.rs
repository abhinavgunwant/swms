pub mod image;
pub mod auth;
pub mod user;
pub mod project;
pub mod rendition;
pub mod role;
pub mod folder;

use actix_web::{ HttpResponse, HttpRequest, get, web::Data };
use serde::Serialize;
use qstring::QString;
use log::{ debug, error };

use crate::{
    api::service::path::split_path, server::db::DBError, auth::AuthMiddleware,
    repository::Repository,
    model::{ image::Image, folder::Folder, rendition::Rendition },
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    success: bool,
    message: String,
}

#[derive(Serialize)]
pub struct GetChildrenResponse {
    folders: Vec<Folder>,
    images: Vec<Image>,
    rendition: Option<Rendition>,
    success: bool,
    message: Vec<String>,
}

#[derive(PartialEq)]
pub enum ResourceType {
    Project,
    //Image,
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
 * - `show-all` - Optional. Default is `false`. Shows all children when `true`.
 *
 * e.g.:
 * /api/admin/get-children?type=<type>&path=<path>
 */
#[get("/api/admin/get-children")]
pub async fn get_children(
    repo: Data<dyn Repository + Sync + Send>,
    req: HttpRequest, _: AuthMiddleware
) -> HttpResponse {
    let qs = QString::from(req.query_string());

    let mut response_msg: Vec<String> = vec![];

    let mut error: bool = false;

    // URL parameter vars:
    let _type: ResourceType;
    let show_all: bool;

    if let Some(show_all_qs) = qs.get("show-all") {
        match show_all_qs.to_uppercase().clone().as_str() {
            "TRUE" => { show_all = true; }
            _ => { show_all = false; }
        }
    } else {
        show_all = false;
    }

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

    let path_segments: Vec<&str>;

    match qs.get("path") {
        Some(s) => { path_segments = split_path(s); }

        None => {
            error = true;

            path_segments = vec![];

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

    let mut img_repo;
    let mut fol_repo;

    //debug!("Path is: {}", path);

    match repo.get_folder_repo() {
        Ok(f_repo) => { fol_repo = f_repo; }
        Err(e) => {
            error!("Error while getting folder repo: {}", e);

            return HttpResponse::InternalServerError()
                .body("Some internal error occured!");
        }
    }

    match repo.get_image_repo() {
        Ok(i_repo) => { img_repo = i_repo; }
        Err(e) => {
            error!("Error while getting image repo: {}", e);

            return HttpResponse::InternalServerError()
                .body("Some internal error occured!");
        }
    }

    let project_slug: String;
    let project_id: u32;
    let mut folder_id: u32 = 0;
    let mut image_id: u32 = 0;

    debug!("Validating project slug: {}", path_segments[0]);

    match repo.get_project_repo() {
        Ok(mut proj_repo) => {
            // Validate project
            match proj_repo.is_valid_slug(path_segments[0].to_owned()) {
                Ok (valid_option) => {
                    match valid_option {
                        Some(id) => {
                            debug!("\t-> Project Valid!");
                            project_slug = String::from(path_segments[0]);
                            project_id = id;
                        },

                        None => {
                            return HttpResponse::NotFound().body("NOT FOUND");
                        }
                    }
                }

                Err (err) => {
                    error!("Some error occured while getting project {}", err);

                    return HttpResponse::InternalServerError().body(
                        "Some error occured, please try again later!"
                    );
                }
            }
        }

        Err(err) => {
            error!("Some error occured while getting project {}", err);

            return HttpResponse::InternalServerError().body(
                "Some error occured, please try again later!"
            );
        }
    }

    // if only project slug is supplied, return project object, otherwise
    // iterate and verify until last path_segment and return the last object.
    if path_segments.len() == 1
        && (_type == ResourceType::Project || _type == ResourceType::NONE)
    {
        debug!("show_all: {}", show_all);

        return generate_resource_response(
            fol_repo.get_from_project_slug(String::from(path_segments[0]), show_all),
            img_repo.get_from_project_slug(String::from(path_segments[0]), show_all),
            ResourceType::Project,
        );
    } else {
        for (i, path_segment) in path_segments[1..path_segments.len()].into_iter().enumerate() {
            let is_last = i == path_segments.len() - 2;
            let path_seg_owned = String::from(*path_segment);
            let mut resource_found = false;

            debug!("\tChecking folder with slug: {}", path_seg_owned.clone());

            // The last slug is usually the image rendition slug.
            if is_last && image_id != 0
                && (
                    _type == ResourceType::Rendition
                    || _type == ResourceType::NONE
            ) {
                // Check if rendition slug
                debug!("Validating rendition slug: {}", path_segment);
                match repo.get_rendition_repo() {
                    Ok(mut ren_repo) => {
                        match ren_repo.get_from_project_rendition_slug(
                            project_slug.clone(),
                            path_seg_owned.clone()
                        ) {
                            Ok (rendition) => {
                                debug!("\t-> Returning Rendition!");

                                // TODO: Check if the user has access.
                                return HttpResponse::Ok().json(GetChildrenResponse {
                                    images: vec![],
                                    folders: vec![],
                                    success: true,
                                    rendition: Some(rendition),
                                    message: vec![ String::from("RENDITION") ],
                                });
                            }

                            Err (_) => {}
                        }
                    }

                    Err(e) => {
                        error!("Error while getting rendition repository: {}", e);
                    }
                }
            }

            // Check if folder
            match fol_repo.is_valid_slug(
                project_id, folder_id, path_seg_owned.clone()
            ) {
                Ok (valid_option) => {
                    match valid_option {
                        Some(id) => {
                            if is_last {
                                return generate_resource_response(
                                    fol_repo.get_from_folder_slug(
                                        path_seg_owned.clone(),
                                        show_all,
                                    ),
                                    img_repo.get_from_folder_slug(
                                        path_seg_owned.clone(),
                                        show_all
                                    ),
                                    ResourceType::Folder,
                                );
                            }

                            folder_id = id;
                            resource_found = true;
                        }

                        None => {}
                    }
                }

                Err (e) => {
                    match e {
                        DBError::OtherError => {
                            return HttpResponse::InternalServerError().json(GetChildrenResponse {
                                images: vec![],
                                folders: vec![],
                                success: false,
                                rendition: None,
                                message: vec![
                                    String::from("Some error occured, please try again later!")
                                ],
                            });
                        }

                        _ => {}
                    }
                }
            }

            debug!("\tChecking image with slug: {}", path_seg_owned.clone());

            // Check if image
            match img_repo.is_valid_slug(
                project_id, folder_id, path_seg_owned.clone()
            ) {
                Ok (valid_option) => {
                    match valid_option {
                        Some(id) => {
                            debug!("\tFound image ({}), id: {}", path_seg_owned.clone(), id);
                            if is_last {
                                match img_repo.get(id) {
                                    Ok (image) => {
                                        return HttpResponse::Ok().json(GetChildrenResponse {
                                            images: vec![ image ],
                                            folders: vec![],
                                            success: true,
                                            rendition: None,
                                            message: vec![ String::from("IMAGE") ],
                                        });
                                    }

                                    Err (e) => { error!("{}", e); }
                                }
                            }

                            resource_found = true;
                            image_id = id;
                        }

                        None => {}
                    }
                }

                Err(e) => {
                    match e {
                        DBError::OtherError => {
                            return HttpResponse::InternalServerError().json(
                                GetChildrenResponse {
                                images: vec![],
                                folders: vec![],
                                success: false,
                                rendition: None,
                                message: vec![
                                    String::from("Some error occured, please \
                                        try again later!")
                                ],
                            });
                        }

                        _ => {}
                    }
                }
            }

            if !resource_found {
                return HttpResponse::NotFound().body("NOT FOUND");
            }
        }
    }

    HttpResponse::NotFound().json(GetChildrenResponse {
        images: vec![],
        folders: vec![],
        success: false,
        rendition: None,
        message: vec![ String::from("Error: Not Found") ],
    })
}

/// Generates a response based on the folders and images provided.
///
/// # Arguments
///
/// - `folders_wrapped` the result containing folder vector.
/// - `images_wrapped` the result containing images vector.
/// - `resource_type` the type of parent resource for which this response is
///   being generated. Should only be `ResourceType::Folder` or
///   `ResourceType::Project`.
fn generate_resource_response(
    folders_wrapped: Result<Vec<Folder>, DBError>,
    images_wrapped: Result<Vec<Image>, DBError>,
    resource_type: ResourceType,
) -> HttpResponse {
    let mut error: bool = false;
    let mut images_found: bool = false;
    let mut folders_found: bool = false;

    let mut response_images: Vec<Image> = vec![];
    let mut response_folders: Vec<Folder> = vec![];
    let mut response_msg: Vec<String> = vec![];

    // collect images
    match images_wrapped {
        Ok (images) => {
            debug!("Found wrapped images.");
            response_images = images;

            if !response_images.is_empty() {
                images_found = true;
            }
        }

        Err (e) => {
            match e {
                DBError::NotFound => {}
                _ => {
                    error!("Some internal error occured while fetching \
                        project images: {}", e);

                    response_msg.push(String::from(
                        "Some internal error occured while fetching images."
                    ));

                    error = true;
                }
            }
        }
    }

    // collect folders
    match folders_wrapped {
        Ok (folders) => {
            debug!("Found wrapped folders.");
            response_folders = folders;

            if !response_folders.is_empty() {
                folders_found = true;
            }
        }

        Err (e) => {
            match e {
                DBError::NotFound => {},
                _ => {
                    error!("Some internal error occured while fetching project folders: {}", e);

                    response_msg.push(String::from(
                        "Some internal error occured while fetching folders."
                    ));

                    error = true;
                }
            }
        }
    }

    if !(images_found || folders_found) {
        let mut msg: String = String::from("");

        if resource_type == ResourceType::Project {
            msg.push_str("Project ");
        } else {
            msg.push_str("Folder ");
        }

        msg.push_str("is empty.");

        response_msg.push(msg);
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
        images: vec![],
        folders: vec![],
        success: true,
        rendition: None,
        message: response_msg,
    })
}

// ------------------------
// Implementations here...
// ------------------------

impl SuccessResponse {
    fn new(success: bool, message: String) -> Self { Self {success,message} }
}

