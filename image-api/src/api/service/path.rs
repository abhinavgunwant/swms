//! Path and slug service

use regex::Regex;

use crate::{
    db::DBError,
    repository::{
        image::{ ImageRepository, get_image_repository },
        folder::{ FolderRepository, get_folder_repository },
        project::{ ProjectRepository, get_project_repository },
        rendition::{ RenditionRepository, get_rendition_repository },
    },
    model::{
        image::Image, folder::Folder, rendition::Rendition, item::Item,
        error::{ Error, ErrorType },
    },
};

pub fn get_rendition_from_path_segments(path_segments: Vec<&str>) -> Result<Rendition, Error> {
    let img_repo = get_image_repository();
    let ren_repo = get_rendition_repository();
    let fol_repo = get_folder_repository();
    let proj_repo = get_project_repository();

    let project_slug: String;
    let project_id: u32;
    let mut folder_id: u32 = 0;
    let mut image_id: u32 = 0;

    println!("Validating project slug: {}", path_segments[0]);

    // Validate project
    match proj_repo.is_valid_slug(path_segments[0].to_owned()) {
        Ok (valid_option) => {
            match valid_option {
                Some(id) => {
                    println!("\t-> Project Valid!");
                    project_slug = String::from(path_segments[0]);
                    project_id = id;
                },

                None => {
                    return Err(Error::new(ErrorType::NotFound, "NOT FOUND"));
                }
            }
        }

        Err (err) => {
            eprintln!("Some error occured while getting project {}", err);

            return Err(Error::new(
                ErrorType::InternalError,
                "Some error occured, please try again later!"
            ));
        }
    }

    // if only project slug is supplied, return not found error
    if path_segments.len() == 1 {
        return Err(Error::new(ErrorType::NotFound, "NOT FOUND"));
    }

    for (i, path_segment) in path_segments[1..path_segments.len()].into_iter().enumerate() {
        let is_last = i == path_segments.len() - 2;
        let mut path_seg_owned = String::from(*path_segment);
        let mut resource_found = false;

        println!("\tChecking folder with slug: {}", path_seg_owned.clone());

        if is_last && match_extension(path_segment) {
            // TODO: Extract the extension here and match it with the rendition
            path_seg_owned =  String::from(
                path_seg_owned.split(".").collect::<Vec<_>>()[0]
            );
        }

        // The last slug is usually the image rendition slug.
        if is_last && image_id != 0 {
            // Check if rendition slug
            println!("\tValidating rendition slug: {}", path_segment);
            match ren_repo.get_from_image_and_slug(
                image_id,
                path_seg_owned.clone()
            ) {
                Ok (rendition) => {
                    println!("\t\t-> Returning Rendition!");

                    return Ok(rendition);
                }

                Err (_) => {}
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
                            return Err(Error::new(ErrorType::NotFound, "NOT FOUND"));
                        }

                        folder_id = id;
                        resource_found = true;
                    }

                    None => {}
                }
            }

            Err (e) => {
                if e == DBError::OtherError {
                    return Err(Error::new(ErrorType::InternalError, "Some error occured"));
                }
            }
        }

        println!("\tChecking image with slug: {}", path_seg_owned.clone());

        // Check if image
        match img_repo.is_valid_slug(
            project_id, folder_id, path_seg_owned.clone()
        ) {
            Ok (valid_option) => {
                match valid_option {
                    Some(id) => {
                        println!("\t\t-> Found image ({}), id: {}", path_seg_owned.clone(), id);
                        if is_last {
                            match ren_repo.get_from_image_and_slug(
                                id, String::from("default")
                                ) {
                                Ok (rendition) => {
                                    println!("\t\t-> Returning default rendition");
                                    return Ok(rendition);
                                }

                                Err (e) => {
                                    if e == DBError::NOT_FOUND {
                                        return Err(Error::new(
                                            ErrorType::NotFound, "NOT FOUND"
                                        ));
                                    }

                                    return Err(Error::new(
                                        ErrorType::InternalError,
                                        "Some error occured"
                                    ));
                                }
                            }
                        }

                        resource_found = true;
                        image_id = id;
                    }

                    None => {}
                }
            }

            Err(e) => {
                if e == DBError::OtherError {
                    return Err(Error::new(ErrorType::InternalError, "Some error occured"));
                }
            }
        }

        if !resource_found {
            return Err(Error::new(ErrorType::NotFound, "NOT FOUND"));
        }
    }

    Err(Error::new(ErrorType::NotFound, "NOT FOUND"))
}

/// Takes raw path as input and returns vector containing path segments.
pub fn split_path(path: &str) -> Vec<&str> {
    let mut chars = path.chars();
    let mut updated = false;

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
        chars.as_str().split("/").collect()
    } else {
        path.split("/").collect()
    }
}

/// Returns true if a provided string contains an extension
/// e.g.: "example.jpg"
fn match_extension(text: &str) -> bool {
    if text.is_empty() { return false; }

    let re = Regex::new(r"\.(png|gif|jpg|jpeg|webp|tif|bmp|raw|cr2|nef|orf|sr2|eps|svg)$").unwrap();
    re.is_match(text)
}

