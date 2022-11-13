// use std::path::PathBuf;
use std::io::Write;
use std::fs::{ File, create_dir_all, read };
use std::path::Path;
use std::time::{ Duration, Instant };
use actix_multipart::Multipart;
use actix_web::{ get, post, web, HttpResponse, HttpRequest };
// use actix_form_data::{ handle_multipart, Error, Field, Form, Value };
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
use serde::{ Serialize, Deserialize };
use raster;
use regex::Regex;
use crate::{
    db::DBError,
    repository::{
        rendition::{
            RenditionRepository, get_rendition_repository
        },
        image::{ ImageRepository, get_image_repository },
    },
    model::rendition::Rendition,
};

#[derive(Serialize)]
pub struct ImageJson {
    slug: String,
    id: u32,
}

#[post("/api/image")]
pub async fn upload(mut payload: Multipart, file_path: String) -> HttpResponse {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let filename = format!("image-uploads/{}.jpg", Uuid::new_v4());

        let path = Path::new(String::as_str(&filename));
        let parents = path.parent().unwrap();

        create_dir_all(parents).unwrap();

        println!("Created file: {}", filename);

        // File::create is blocking operation, use threadpool
        let mut file = web::block(move || File::create(filename))
            .await
            .unwrap()
            .expect("error");

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            file = web::block(move || file.write_all(&data).map(|_| file))
                .await
                .unwrap()
                .expect("error");
        }
    }

    return HttpResponse::Ok().body("!!");
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
#[get("/api/image/{path:[/\\.\\-+a-zA-Z0-9\\(\\)]+(\\.\\w{2,5})$}")]
pub async fn download(req: HttpRequest) -> HttpResponse {
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

    let mut rendition_slug: String = String::new();

    // Tells whether the requested image has extension
    let mut img_has_ext: bool = false;

    let repo = get_rendition_repository();
    let img_repo = get_image_repository();

    if path_list_len.clone() == 2 {
        img_has_ext = img_ext_pat.is_match(path_list[1]);

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
                    "image-rendition-cache/{}{}",
                    rendition.id,
                    rendition.encoding.extension()
                );

                if !Path::new(dest_file_path.as_str()).exists() {
                    // Get source image element from the rendition
                    // to get the source image extension
                    match img_repo.get(rendition.image_id) {
                        Ok (image_data) => {
                            let source_file_path = format!(
                                "image-uploads/{}{}",
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
                
                let image_file = web::block(
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

#[get("/api/imagedata/{image_id}")]
pub async fn imagedata(req: HttpRequest) -> HttpResponse {
    let image_id:u32 = req.match_info().get("image_id").unwrap().parse()
        .unwrap();

    match get_image_repository().get(image_id) {
        Ok (image) => {
            println!("got id: {}, name: {}", image.id, image.name);
            HttpResponse::Ok().json(image)
        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound().body("Not Found");
            }

            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
