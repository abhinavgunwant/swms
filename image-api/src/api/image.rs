// use std::path::PathBuf;
use std::io::Write;
use std::fs::{ File, read };
use std::path::{Path, self};
//use std::time::{ Duration, Instant };
use actix_multipart::Multipart;
use actix_web::{
    get, post, web::block, HttpResponse, HttpRequest,
};
// use actix_form_data::{ handle_multipart, Error, Field, Form, Value };
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
use serde::Serialize;
use raster;
use regex::Regex;
use crate::{
    api::{
        DEST_REN_DIR, IMG_UPL_DIR,
        service::path::{
            get_rendition_from_path_segments, split_path, create_folder_tree,
            rendition_cache_path,
        }
    },
    repository::{
        rendition::{ RenditionRepository, get_rendition_repository },
        image::{ ImageRepository, get_image_repository },
    },
    model::{ rendition::Rendition, error::ErrorType, encoding::Encoding },
    db::DBError,
};

#[derive(Serialize)]
pub struct ImageJson {
    slug: String,
    id: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageUploadResponse {
    success: bool,
    upload_id: String,
    message: String,
}

fn not_found_response() -> HttpResponse {
    HttpResponse::NotFound().body("404: Not Found!")
}

fn error_response(msg: &str) -> HttpResponse {
    if msg.is_empty() {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    }

    HttpResponse::BadRequest().body(String::from(msg))
}

#[post("/api/image")]
pub async fn upload(mut payload: Multipart) -> HttpResponse {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let cd = field.content_disposition();
        let cd_name = String::from(cd.get_name().unwrap());

        match cd.get_filename() {
            Some(filename) => {
                println!("filename: {}", filename);
            }

            None => {}
        }

        if cd_name.eq("payload") {
            let uuid: String = Uuid::new_v4().to_string();
            // "image-uploads/{}.jpg" for final path
            let fname: String = format!("temp/{}.jpg", uuid);

            let mut file = block(move || File::create(fname)).await
                .unwrap()
                .expect("error");

            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                file = block(move || file.write_all(&data).map(|_| file)).await
                    .unwrap()
                    .expect("error");
            }

            return HttpResponse::Ok().json(ImageUploadResponse {
                success: true,
                upload_id: uuid,
                message: String::from(""),
            });
        }
    }

    HttpResponse::InternalServerError().json(ImageUploadResponse {
        success: false,
        upload_id: String::from(""),
        message: String::from("Some error occured while uploading..."),
    })
}

/**
 * Returns the requested image rendition.
 * 
 * Creates a file in the image-rendition-cache folder when the first request is
 * received and returns this cached file in the subsequent requests.
 * 
 * TODO: Make the cache behaviour optional with user choosing whether to make
 * renditions render eagerly or lazily.
 */
pub async fn download2(req: HttpRequest) -> HttpResponse {
    let path: String = req.match_info().get("path").unwrap().parse().unwrap();

    // Refine path string
    let mut path_refined: String = String::from(
        path.as_str().trim_matches('/')
    );
    while path_refined.contains("//") {
        path_refined = path_refined.replace("//", "/");
    }

    let path_list: Vec<&str> = path_refined.split("/").collect();
    let path_list_len = path_list.len();

    let img_ext_pat = Regex::new(r"\.(jpg|jpeg|gif|png|bmp)$").unwrap();


    // Tells whether the requested image has extension

    let repo = get_rendition_repository();
    let img_repo = get_image_repository();

    if path_list_len.clone() == 2 {
        let img_has_ext: bool = img_ext_pat.is_match(path_list[1]);
        let rendition_slug: String;

        if img_has_ext {
            rendition_slug = String::from(
                img_ext_pat.replace_all(path_list[1], "").as_ref()
            );
        } else {
            rendition_slug = String::from(path_list[1]);
        }

        let rendition_result: Result<Rendition, DBError> = repo.
            get_from_project_rendition_slug(
                String::from(path_list[0]),
                rendition_slug
            );

        match rendition_result {
            Ok (rendition) => {
                // TODO: Check if img_has_ext is true, if it is, check if it
                //    has any renditions that match it's extention.");

                let dest_file_path = format!(
                    "{}/{}{}",
                    DEST_REN_DIR,
                    rendition.id,
                    rendition.encoding.extension()
                );

                if !Path::new(dest_file_path.as_str()).exists() {
                    // Get source image element from the rendition
                    // to get the source image extension
                    match img_repo.get(rendition.image_id) {
                        Ok (image_data) => {
                            let source_file_path = format!(
                                "{}/{}{}",
                                IMG_UPL_DIR,
                                image_data.id,
                                image_data.encoding.extension()
                            );

                            println!("Getting source file: {}", source_file_path);

                            let mut raster_img = raster::open(
                                source_file_path.as_str()
                            ).unwrap();

                            raster::editor::resize(
                                &mut raster_img,
                                rendition.width as i32,
                                rendition.height as i32,
                                raster::ResizeMode::Fit
                            ).unwrap();

                            raster::save(
                                &raster_img,
                                dest_file_path.as_str()
                            ).unwrap();
                        }

                        Err (e) => {
                            if e == DBError::NOT_FOUND {
                                return HttpResponse::NotFound()
                                    .body("Not Found");
                            }

                            if e == DBError::OtherError {
                                return HttpResponse::InternalServerError()
                                    .body("Internal Server Error");
                            }
                        }
                    }
                }
                
                let image_file = block(
                    move || read(String::from(dest_file_path))
                ).await.unwrap().expect("Error whie downloading!");

                return HttpResponse::Ok()
                    .content_type(rendition.encoding.mime_type())
                    .body(image_file);
            }

            Err (e) => {
                if e == DBError::NOT_FOUND {
                    return HttpResponse::NotFound().body("Not Found!!!!");
                }

                return HttpResponse::InternalServerError()
                    .body("Some error occured");
            }
        }
    }

    if path_list_len > 2 {
        return HttpResponse::Ok().body("Image child of a folder");
    }

    HttpResponse::NotFound().body("Not Found")
}

#[get("/api/image/{path:[/\\.\\-+a-zA-Z0-9\\(\\)]+(\\.\\w{2,5})?$}")]
pub async fn download(req: HttpRequest) -> HttpResponse {
    if let Some(path) = req.match_info().get("path") {
        println!("Requested Path: \"{}\"", path);

        let mut dest_file_path: String = format!("{}/{}", DEST_REN_DIR, path);
        let mime_type: String;

        match rendition_cache_path(&dest_file_path) {
            Some(p) => {
                println!("--> Found in rendition cache!");

                if p != dest_file_path {
                    println!("--> Updating file path with: {}", p);
                    dest_file_path = p;
                }

                mime_type = Encoding::from(dest_file_path.as_str())
                    .mime_type();
            }

            None => {
                let mut path_segments = split_path(path);

                match get_rendition_from_path_segments(&path_segments) {
                    Ok(rendition) => {
                        match get_image_repository().get(rendition.image_id) {
                            Ok (image_data) => {
                                let source_file_path = format!(
                                    "{}/{}{}",
                                    IMG_UPL_DIR,
                                    image_data.id,
                                    image_data.encoding.extension()
                                );

                                let src_img_path = source_file_path.as_str();

                                println!(
                                    "Getting source file: {}",
                                    source_file_path
                                );

                                match raster::open(src_img_path) {
                                    Ok(mut raster_img) => {
                                        let ren_slug = rendition.slug.as_str();
                                        let end_indx = path_segments.len() - 1;
                                        dest_file_path = format!(
                                            "{}/{}", DEST_REN_DIR, path
                                        );

                                        let ext = rendition.encoding
                                            .extension();
                                        let ext_str = ext.as_str();
                                        // "end of path_segment" :)
                                        let eps = path_segments[end_indx];

                                        // Check if supplied path contains slug
                                        // and file extension.
                                        if eps.contains(ren_slug) {
                                            path_segments.remove(end_indx);

                                            if !eps.ends_with(ext_str) {
                                                println!("pushing rendition slug into file name");
                                                dest_file_path.push_str(ext_str);
                                            }
                                        } else {
                                            dest_file_path.push('/');
                                            dest_file_path.push_str(
                                                rendition.slug.as_str()
                                            );

                                            if !eps.ends_with(ext_str) {
                                                println!("pushing rendition slug into file name");
                                                dest_file_path.push_str(ext_str);
                                            }
                                        }

                                        raster::editor::resize(
                                            &mut raster_img,
                                            rendition.width as i32,
                                            rendition.height as i32,
                                            raster::ResizeMode::Fit
                                        ).unwrap();

                                        match create_folder_tree(
                                            DEST_REN_DIR, path_segments
                                        ) {
                                            Err (()) => {
                                                return error_response("");
                                            }
                                            _ => {}
                                        }

                                        println!("Saving rendition to path: {}", dest_file_path);
                                        raster::save(
                                            &raster_img, &dest_file_path
                                        ).unwrap();

                                        mime_type = rendition.encoding.mime_type();
                                    }

                                    Err(_) => { return error_response(""); }
                                }
                            }

                            Err (e) => {
                                match e {
                                    DBError::NOT_FOUND => {
                                        return not_found_response();
                                    }

                                    _ => { return error_response(""); }
                                }
                            }
                        }
                    }

                    Err (e) => {
                        match e.error_type {
                            ErrorType::NotFound => { return not_found_response(); }
                            ErrorType::InternalError => { return error_response(""); }
                        }
                    }
                }
            }
        }

        let image_file = block(move || read(dest_file_path)).await
            .unwrap().expect("Error while downloading!");

        return HttpResponse::Ok().content_type(mime_type).body(image_file);

    }

    not_found_response()
}

