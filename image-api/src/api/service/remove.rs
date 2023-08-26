//! Delete service

use std::fs::{ read, rename, remove_file };
use crate::{
    db::DBError,
    repository::{
        folder::{ FolderRepository, get_folder_repository },
        image::{ ImageRepository, get_image_repository },
        rendition::{ RenditionRepository, get_rendition_repository },
    },
    model::{ image::Image, rendition::Rendition },
};

/// Removes images in `image_ids`
pub fn remove_images(image_ids: &Vec<u32>) -> Result<String, String> {
    let mut image: Option<Image>;
    let mut error: bool = false;

    let img_repo = get_image_repository();
    let ren_repo = get_rendition_repository();

    for image_id in image_ids.iter() {
        // Get image object
        match img_repo.get(*image_id) {
            Ok (img) => { image = Some(img); }

            Err (e) => {
                error = true;

                if e == DBError::NOT_FOUND {
                    eprintln!("Error in delete image api while getting image object: Image not found");
                } else {
                    eprintln!("Unknown error in delete image api while getting image object!");
                }

                image = None;
            }
        }

        let mut renditions: Vec<Rendition> = vec![];

        match ren_repo.get_all_from_image(*image_id) {
            Ok (rens) => { renditions = rens; }
            Err (_) => {}
        }

        if !renditions.is_empty() {
            match ren_repo.remove_all_from_image(*image_id) {
                Ok (msg) => {
                    println!("{} from database!", msg);

                    for rendition in renditions.iter() {
                        let file_name: String = format!(
                            "image-rendition-cache/{}{}",
                            rendition.id,
                            rendition.encoding.extension(),
                        );

                        match remove_file(file_name.clone()) {
                            Ok (_) => {}

                            Err (e) => {
                                eprintln!(
                                    "Error while deleting rendition file {} (id: {}) for image id: {}: {}",
                                    file_name,
                                    rendition.id,
                                    image_id,
                                    e
                                );

                                error = true;
                            }
                        }
                    }
                }

                Err (e_msg) => {
                    eprintln!("{}", e_msg);

                    return Err(format!(
                        "Couldn't remove renditions for image (id: {})",
                        image_id
                    ));
                }
            }
        }

        match get_image_repository().remove_item(*image_id) {
            Ok (_message) => {
                // Delete the image file if it exists
                match image {
                    Some (img) => {
                        match remove_file (
                            format!(
                                "image-uploads/{}{}",
                                image_id,
                                img.encoding.extension()
                        )) {
                            Ok (_) => {}

                            Err (e) => {
                                eprintln!("Error while deleting image file for image id: {}: {}", image_id, e);
                                error = true;
                            }
                        }
                    }

                    None => {}
                }
            }

            Err (_err_msg) => { error = true; }
        }
    }

    if error {
        return Err(String::from("Some images could not be removed successfully"));
    } else {
        return Ok (String::from("Removed images successfully"));
    }
}

/// Removes all folders and all their children under the folder hierarchy.
///
/// This is how it works for each item (folder id) in the list:
/// 1. Removes the folder
/// 2. Removes the images in the removed folder.
/// 3. Gets the sub-folders of the removed folder
/// 4. Repeat 1 with the subfolders
///
/// ### Parameters
/// - `folder_ids`: IDs of folders to be deleted
pub fn remove_folders(folder_ids: &mut Vec<u32>) -> Result<String, String> {
    let mut error: bool = false;

    let fol_repo = get_folder_repository();
    let img_repo = get_image_repository();

    let mut folders_to_delete: Vec<u32> = vec![];

    folders_to_delete.append(folder_ids);

    for fid in folder_ids.iter() {
        folders_to_delete.append(&mut get_subfolders(*fid));
    }

    for folder_id in folders_to_delete.iter() {
        match fol_repo.remove_item(*folder_id) {
            Ok (_) => {
                match img_repo.get_from_folder(*folder_id, true) {
                    Ok (images) => {
                        let mut image_ids: Vec<u32> = vec![];

                        for image in images.iter() {
                            image_ids.push(image.id);
                        }

                        match remove_images(&image_ids) {
                            Ok (_) => {}
                            Err (_) => { error = true; }
                        }
                    }

                    Err (_) => { error = true; }
                }
            },
            Err (_) => { error = true; },
        }
    }

    if error {
        Err (String::from("Error while removing folders"))
    } else {
        Ok (String::from("Folders removed"))
    }
}

fn get_subfolders(folder_id: u32) -> Vec<u32> {
    let mut f_ids: Vec<u32> = vec![];

    match get_folder_repository().get_from_folder(folder_id) {
        Ok (folders) => {
            for f in folders.iter() {
                f_ids.push(f.id);

                f_ids.append(&mut get_subfolders(f.id));
            }
        }

        Err (e) => { println!("{}", e); }
    }

    return f_ids;
}

