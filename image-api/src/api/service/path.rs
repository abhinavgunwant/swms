//! Path and slug service

use std::{ fs::create_dir_all, path::Path };
use raster;
use log::{ debug, error };

use crate::{
    db::DBError,
    repository::{
        image::{ ImageRepository, get_image_repository },
        folder::{ FolderRepository, get_folder_repository },
        project::{ ProjectRepository, get_project_repository },
        rendition::{ RenditionRepository, get_rendition_repository },
    },
    model::{
        rendition::Rendition, error::{ Error, ErrorType }, image::Image,
        encoding::{ Encoding, RE },
    },
};

/// Gets rendition that is represented by the given path
pub fn get_rendition_from_path_segments<'a >(path_segments: &'a Vec<&str>)
    -> Result<Rendition, Error<'a>> {
    let img_repo = get_image_repository();
    let ren_repo = get_rendition_repository();
    let fol_repo = get_folder_repository();
    let proj_repo = get_project_repository();

    let project_id: u32;
    let mut folder_id: u32 = 0;
    let mut image_id: u32 = 0;

    debug!("Validating project slug: {}", path_segments[0]);

    // Validate project
    match proj_repo.is_valid_slug(path_segments[0].to_owned()) {
        Ok (valid_option) => {
            match valid_option {
                Some(id) => {
                    debug!("\t-> Project Valid!");
                    project_id = id;
                },

                None => {
                    return Err(Error::new(ErrorType::NotFound, "NOT FOUND"));
                }
            }
        }

        Err (err) => {
            error!("Some error occured while getting project {}", err);

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

        debug!("\tChecking folder with slug: {}", path_seg_owned.clone());

        if is_last && Encoding::match_extension(path_segment) {
            // TODO: Extract the extension here and match it with the rendition
            path_seg_owned =  String::from(
                path_seg_owned.split(".").collect::<Vec<_>>()[0]
            );
        }

        // The last slug is usually the image rendition slug.
        if is_last && image_id != 0 {
            // Check if rendition slug
            debug!("\tValidating rendition slug: {}", path_segment);
            match ren_repo.get_from_image_and_slug(
                image_id,
                path_seg_owned.clone()
            ) {
                Ok (rendition) => {
                    debug!("\t\t-> Returning Rendition!");

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

        debug!("\tChecking image with slug: {}", path_seg_owned.clone());

        // Check if image
        match img_repo.is_valid_slug(
            project_id, folder_id, path_seg_owned.clone()
        ) {
            Ok (valid_option) => {
                match valid_option {
                    Some(id) => {
                        debug!("\t\t-> Found image ({}), id: {}", path_seg_owned.clone(), id);
                        if is_last {
                            match ren_repo.get_from_image_and_slug(
                                id, String::from("default")
                                ) {
                                Ok (rendition) => {
                                    debug!("\t\t-> Returning default rendition");
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

/// Gets the path of an image
pub fn get_image_path(image: Image) -> Result<String, DBError> {
    debug!("Getting image path");
    let fol_repo = get_folder_repository();
    let prj_repo = get_project_repository();

    let mut path: String = image.slug;

    let mut folder_id: u32 = image.folder_id;

    while folder_id != 0 {
        match fol_repo.get(folder_id) {
            Ok(folder) => {
                folder_id = folder.parent_folder_id;
                path = format!("{}/{}", folder.slug, path);
            }

            Err(e) => {
                error!("Error while generating image path1: {}, folder: {}", e, folder_id);
                return Err(e);
            }
        }
    }

    debug!("-> Getting project");

    match prj_repo.get(image.project_id) {
        Ok(project) => {
            path = format!("{}/{}", project.slug, path);
        }

        Err(e) => {
            error!("Error while generating image path2: {}", e);
            return Err(e);
        }
    }

    debug!("-> done!");

    Ok(path)
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

/// Creates folder tree on the file system for the path supplied.
pub fn create_folder_tree(path: &str) -> Result<(), ()> {
    debug!("in create_folder_tree");
    let mut path_updated = String::new();
    let mut insert_slash = false;

    for p in split_path(path).iter() {
        if Encoding::match_extension(p) {
            break;
        }

        if insert_slash {
            path_updated.push('/');
        }

        path_updated.push_str(p);
        insert_slash = true;
    }

    let path_str = path_updated.as_str();

    debug!("  -> checking if \"{}\" exists.", path_str);
    if !Path::new(path_str).exists() {
        match create_dir_all(path_str) {
            std::io::Result::Ok(()) => {
                debug!("  -> Tree created.");
                return Ok(());
            }
            std::io::Result::Err(e) => {
                error!("Error occured while creating renditions: {}", e);
                return Err(());
            }
        }
    }

    debug!("  -> Tree exists already.");

    Ok (())
}

/// Returns the path where the rendition cache exists.
/// `None` if rendition cache does not exist.
pub fn rendition_cache_path(path: &str) -> Option<String> {
    if path.is_empty() { return None; }

    let mut def_img_path: String = String::from(path);

    if Encoding::match_extension(path) {
        if Path::new(path).exists() {
            return Some(String::from(path));
        } else {
            def_img_path = String::from(RE.replace(path, ""));
        }
    }

    def_img_path = format!("{}/default.jpg", def_img_path);

    if Path::new(def_img_path.as_str()).exists() {
        return Some(def_img_path);
    }

    for enc in Encoding::iter() {
        let ext = enc.extension();

        if ext.is_empty() { continue; }

        let new_path = format!("{}{}", path, ext);

        debug!("Checking if {} exists", new_path);

        if Path::new(new_path.as_str()).exists() {
            return Some(new_path);
        }
    }

    None
}

pub fn resize_and_save_rendition(
    raster_img: &mut raster::Image, dest_path: &str, width: u16, height: u16
) -> Result<(),()> {
    match raster::editor::resize(
        raster_img,
        width as i32,
        height as i32,
        raster::ResizeMode::Fit
    ) {
        Ok(_) => {
            match create_folder_tree(dest_path) {
                Err (()) => { return Err(()); }
                _ => {}
            }

            debug!("Saving rendition to path: {}", dest_path);

            match raster::save(&raster_img, dest_path) {
                Ok (_) => { return Ok (()); }
                Err(_) => {
                    error!("Error while saving file.");
                    return Err(());
                }
            }
        }

        Err(_) => { error!("Error while resizing."); return Err(()); }
    }
}

pub fn cache_rendition_file(
    src_path: &str, dest_path: &str, width: u16, height: u16
) -> Result<(),()> {
    match raster::open(src_path) {
        Ok(mut raster_img) => {
            return resize_and_save_rendition(
                &mut raster_img, dest_path, width, height
            );
        }

        Err(_) => Err(())
    }
}

