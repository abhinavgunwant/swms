use std::{ io::Write, fs::{ File, read }, sync::Mutex, ops::Deref };

use actix_multipart::Multipart;
use actix_web::{ get, post, web::{ block, Data }, HttpResponse, HttpRequest };
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
use serde::Serialize;
use log::debug;

use crate::{
    api::service::path::{
        get_rendition_from_path_segments, split_path,
        rendition_cache_path, cache_rendition_file,
    },
    repository::image::{ ImageRepository, get_image_repository },
    model::{ error::ErrorType, encoding::Encoding },
    db::DBError, auth::AuthMiddleware, server::config::ServerConfig,
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
pub async fn upload(mut payload: Multipart, _: AuthMiddleware)
    -> HttpResponse {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let cd = field.content_disposition();
        let cd_name = String::from(cd.get_name().unwrap());

        match cd.get_filename() {
            Some(filename) => {
                debug!("filename: {}", filename);
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

#[get("/api/image/{path:[/\\.\\-+a-zA-Z0-9\\(\\)]+(\\.\\w{2,5})?$}")]
pub async fn download(req: HttpRequest, conf: Data<Mutex<ServerConfig>>)
    -> HttpResponse {
    if let Some(path) = req.match_info().get("path") {
        debug!("Requested Path: \"{}\"", path);

        let c_ = conf.lock().unwrap();
        let config: &ServerConfig = c_.deref();

        let mut dest_file_path: String = format!(
            "{}/{}", &config.rendition_cache_dir, path
        );

        debug!("dest_file_path: {}", dest_file_path);

        let mime_type: String;

        match rendition_cache_path(&dest_file_path) {
            Some(p) => {
                debug!("--> Found in rendition cache!");

                if p != dest_file_path {
                    debug!("--> Updating file path with: {}", p);
                    dest_file_path = p;
                }

                mime_type = Encoding::from(dest_file_path.as_str())
                    .mime_type();
            }

            None => {
                let path_segments = split_path(path);

                match get_rendition_from_path_segments(&path_segments) {
                    Ok(rendition) => {
                        match get_image_repository().get(rendition.image_id) {
                            Ok (image_data) => {
                                let source_file_path = format!(
                                    "{}/{}{}",
                                    config.upload_dir,
                                    image_data.id,
                                    image_data.encoding.extension()
                                );

                                debug!(
                                    "Getting source file: {}",
                                    source_file_path
                                );

                                let ren_slug = rendition.slug.as_str();
                                let end_indx = path_segments.len() - 1;

                                let ext = rendition.encoding.extension();
                                let ext_str = ext.as_str();
                                // "end of path_segment" :)
                                let eps = path_segments[end_indx];

                                // Check if supplied path contains slug
                                // and file extension.
                                if !eps.contains(ren_slug) {
                                    dest_file_path.push('/');
                                    dest_file_path.push_str(
                                        rendition.slug.as_str()
                                    );
                                }

                                if !eps.ends_with(ext_str) {
                                    dest_file_path.push_str(ext_str);
                                }

                                mime_type = rendition.encoding.mime_type();

                                match cache_rendition_file(
                                    &source_file_path,
                                    &dest_file_path,
                                    rendition.width,
                                    rendition.height
                                    ) {
                                    Ok(_) => {}
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

