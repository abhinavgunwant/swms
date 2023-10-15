//! Delete service

use std::fs::{ remove_file, remove_dir_all };

use actix_web::web::Data;
use log::{ debug, error, info };

use crate::{
    api::service::path::get_image_path, repository::Repository,
    model::{ image::Image, rendition::Rendition }, server::db::DBError,
};

/// Removes images in `image_ids`
pub fn remove_images(
    repo: &Data<dyn Repository + Sync + Send>, image_ids: &Vec<u32>, rendition_path: String,
    upload_path: String,
) -> Result<String, String> {
    let mut image: Option<Image>;
    let mut error: bool = false;

    let mut img_repo;
    let mut ren_repo;

    match repo.get_image_repo() {
        Ok(i_repo) => { img_repo = i_repo; }
        Err(e) => {
            let msg = "Error while getting image repo";
            error!("{}: {}", msg, e);

            return Err(format!("{}.", msg));
        }
    }

    match repo.get_rendition_repo() {
        Ok(r_repo) => { ren_repo = r_repo; }
        Err(e) => {
            let msg = "Error while getting image repo";
            error!("{}: {}", msg, e);

            return Err(format!("{}.", msg));
        }
    }

    for image_id in image_ids.iter() {
        // Get image object
        match img_repo.get(*image_id) {
            Ok (img) => { image = Some(img); }
            Err (e) => {
                error = true;

                match e {
                    DBError::NotFound => {
                        error!("Error in delete image api while getting image \
                            object: Image not found");
                    }

                    _ => {
                        error!("Unknown error in delete image api while \
                            getting image object!");
                    }
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
                Ok (msg) => { debug!("{} from database!", msg); }
                Err (e_msg) => {
                    error!("{}", e_msg);

                    return Err(format!(
                        "Couldn't remove renditions for image (id: {})",
                        image_id
                    ));
                }
            }
        }

        match img_repo.remove_item(*image_id) {
            Ok (_message) => {
                if let Some(img) = image {
                    if let Ok(image_path) = get_image_path(&repo, &img) {
                        // Delete the image file from upload directory
                        match remove_file(format!("{}/{}{}",
                            upload_path, image_id, img.encoding.extension()
                        )) {
                            Ok (_) => {}
                            Err (e) => {
                                error!("Error while deleting image file for \
                                    image id: {}: {}", image_id, e);
                                error = true;
                            }
                        }

                        // Delete the image directory
                        match remove_dir_all(
                            format!("{}/{}", rendition_path, image_path
                        )) {
                            Ok(_) => {
                                info!(
                                    "Deleted image directory: {}",
                                    image_path
                                );
                            }

                            Err(e) => {
                                error!(
                                    "Error deleting image directory ({}): {}",
                                    image_path, e
                                );

                                error = true;
                            }
                        }
                    }
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
pub fn remove_folders(
    repo: Data<dyn Repository + Sync + Send>, folder_ids: &mut Vec<u32>,
    rendition_path: String, upload_path: String,
) -> Result<String, String> {
    let mut error: bool = false;

    let mut fol_repo;
    let mut img_repo;

    match repo.get_folder_repo() {
        Ok(f_repo) => { fol_repo = f_repo; }
        Err(e) => {
            let msg = "Error while getting folder repo";
            error!("{}: {}", msg, e);

            return Err(format!("{}.", msg));
        }
    }

    match repo.get_image_repo() {
        Ok(i_repo) => { img_repo = i_repo },
        Err(e) => {
            let msg = "Error while getting image repo";
            error!("{}: {}", msg, e);

            return Err(format!("{}.", msg));
        }
    }

    let mut folders_to_delete: Vec<u32> = vec![];

    folders_to_delete.append(folder_ids);

    for fid in folder_ids.iter() {
        folders_to_delete.append(&mut get_subfolders(&repo, *fid));
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

                        match remove_images(
                            &repo,
                            &image_ids,
                            rendition_path.clone(),
                            upload_path.clone(),
                        ) {
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

pub fn remove_rendition_file(
    ren_dir: &String, image_path: &String, rendition: &Rendition
) -> bool {
    let file_name: String = format!(
        "{}/{}/{}{}",
        ren_dir,
        image_path,
        rendition.slug,
        rendition.encoding.extension(),
    );

    debug!("Deleting rendition (id: {}) file: {}", rendition.id, file_name);

    match remove_file(&file_name) {
        Ok(_) => true,
        Err(e) => {
            error!(
                "Error while deleting rendition file {} (id: {}): {}",
                file_name, rendition.id, e
            );

            return false;
        }
    }
}

fn get_subfolders(repo: &Data<dyn Repository + Sync + Send>, folder_id: u32) -> Vec<u32> {
    let mut f_ids: Vec<u32> = vec![];

    if let Ok(mut fol_repo) = repo.get_folder_repo() {
        match fol_repo.get_from_folder(folder_id) {
            Ok (folders) => {
                for f in folders.iter() {
                    f_ids.push(f.id);

                    f_ids.append(&mut get_subfolders(repo, f.id));
                }
            }

            Err (e) => { error!("{}", e); }
        }
    }

    return f_ids;
}

