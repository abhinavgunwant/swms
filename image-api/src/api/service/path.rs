//! Path and slug service

use std::{ fs::{ read, create_dir_all }, path::Path };

use actix_web::{
    get, post, web::block, HttpResponse, HttpRequest,
};
use regex::{ Regex, Captures };

use crate::{
    api::{ DEST_REN_DIR, IMG_UPL_DIR },
    db::DBError,
    repository::{
        image::{ ImageRepository, get_image_repository },
        folder::{ FolderRepository, get_folder_repository },
        project::{ ProjectRepository, get_project_repository },
        rendition::{ RenditionRepository, get_rendition_repository },
    },
    model::{
        rendition::Rendition, error::{ Error, ErrorType },
        encoding::{ Encoding, RE }
    },
};

pub fn get_rendition_from_path_segments<'a >(path_segments: &'a Vec<&str>) -> Result<Rendition, Error<'a>> {
    let img_repo = get_image_repository();
    let ren_repo = get_rendition_repository();
    let fol_repo = get_folder_repository();
    let proj_repo = get_project_repository();

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

    RE.is_match(text)
}

fn get_extension(text: &str) -> Option<String> {
    match RE.captures(text) {
        Some(captures) => {
            match captures.get(0) {
                Some(match_) => Some(String::from(match_.as_str())),
                None => None,
            }
        }

        None => None
    }
}

pub fn create_folder_tree(parent_folder: &str, paths: Vec<&str>) -> Result<(), ()> {
    println!("in create_folder_tree");
    let mut path_updated = String::from(parent_folder);

    for p in paths.iter() {
        if match_extension(p) {
            break;
        }

        path_updated.push('/');
        path_updated.push_str(p);
    }

    let path_str = path_updated.as_str();

    println!("  -> checking if \"{}\" exists.", path_str);
    if !Path::new(path_str).exists() {
        match create_dir_all(path_str) {
            std::io::Result::Ok(()) => {
                println!("  -> Tree created.");
                return Ok(());
            }
            std::io::Result::Err(e) => {
                eprintln!("Error occured while creating renditions: {}", e);
                return Err(());
            }
        }
    }

    println!("  -> Tree exists already.");

    Ok (())
}

/// Returns the path where the rendition cache exists.
/// `None` if rendition cache does not exist.
pub fn rendition_cache_path(path: &str) -> Option<String> {
    if path.is_empty() { return None; }

    if match_extension(path) && Path::new(path).exists() {
        return Some(String::from(path));
    }

    let def_img_path = format!("{}/default.jpg", path);

    if Path::new(def_img_path.as_str()).exists() {
        return Some(def_img_path);
    }

    for enc in Encoding::iter() {
        let ext = enc.extension();

        if ext.is_empty() { continue; }

        let new_path = format!("{}{}", path, ext);

        println!("Seeing if {} exists", new_path);

        if Path::new(new_path.as_str()).exists() {
            return Some(new_path);
        }
    }

    None
}

