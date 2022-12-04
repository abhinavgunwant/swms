// use std::path::PathBuf;
use std::io::Write;
use std::fs::{ File, create_dir_all, read, rename, remove_file };
use std::path::Path;
//use std::time::{ Duration, Instant };
use actix_multipart::{ Multipart, Field };
use actix_web::{
    get, post, web::{ Bytes, block }, HttpResponse, HttpRequest,
    http::header::ContentType
};
// use actix_form_data::{ handle_multipart, Error, Field, Form, Value };
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
use serde::{ Serialize, Deserialize };
use raster;
use regex::Regex;
use chrono::Utc;
use crate::{
    db::DBError,
    repository::{
        rendition::{
            RenditionRepository, get_rendition_repository
        },
        image::{ ImageRepository, get_image_repository },
    },
    model::{ image::Image, rendition::Rendition, encoding::Encoding },
};

#[derive(Serialize)]
pub struct ImageJson {
    slug: String,
    id: u32,
}

#[post("/api/image")]
pub async fn upload(mut payload: Multipart) -> HttpResponse {
    let mut name: String = String::from("");
    let mut title: String = String::from("");
    let mut details: String = String::from("");
    let mut project_id: u32 = 0;
    let mut folder_id: u32 = 0;
    let mut filename: String = String::from("");
    //let mut original_filename: String = String::from("");
    let mut file_exists: bool = false;
    // let mut file_data: Vec::<Bytes>;
    //let mut file_data: Vec::<u8> = vec![];
    
    let mut save_image: bool = true;

    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let cd = field.content_disposition();
        let cd_name = cd.get_name().unwrap();

        println!("Currently in: {}", cd_name);

        match cd_name {
            "name" => {
                match cd.get_unknown("name") {
                    Some(n) => {
                        name = String::from(n);
                        println!("Found name in multipart request");
                    },
                    None => {
                        println!("Error in name in multipart request");
                    },
                }
            },

            "title" => {
                match cd.get_unknown("title") {
                    Some (t) => {
                        title = String::from(t);
                        println!("Found tile in multipart request");
                    },
                    None => {
                        println!("Error in title in multipart request");
                    },
                }
            },

            "details" => {
                match cd.get_unknown("details") {
                    Some (d) => {
                        details = String::from(d);
                        println!("Found tile in multipart request");
                    },
                    None => {
                        println!("Error in details in multipart request");
                    },
                }
            },

            "project_id" => {
                match cd.get_unknown("project_id") {
                    Some (pid) => {
                        match pid.parse::<u32>() {
                            Ok (pid_u32) => {
                                project_id = pid_u32;
                                println!("Found project_id in multipart requrest");
                            },
                            Err(_e) => {
                                println!("Error in project_id in multipart request");
                            },
                        }
                    },
                    None => {},
                }
            },

            "folder_id" => {
                match cd.get_unknown("folder_id") {
                    Some (fid) => {
                        match fid.parse::<u32>() {
                            Ok (fid_u32) => {
                                folder_id = fid_u32;
                                println!("Found folder_id in multipart requrest");
                            },
                            Err (_e) => {
                                println!("Error in folder_id in multipart request");
                            },
                        }
                    },
                    None => {},
                }
            },

            "payload" => {
                file_exists = true;

//                match cd.get_filename() {
//                    Some (fname) => original_filename = String::from(fname),
//                    None => {},
//                }
                println!("Found payload in multipart requrest");

                let fname: String = format!("temp/{}.jpg", Uuid::new_v4());
                filename = fname.clone();
                println!("Created file: {}", filename);
                let mut file = block(move || File::create(fname)).await
                    .unwrap()
                    .expect("error");

                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    file = block(move || file.write_all(&data).map(|_| file)).await
                        .unwrap()
                        .expect("error");
                }
            },

            &_ => {
                println!("Kuch bhi...");
            },
        }
    }

    println!("Condition: {}",
        !name.is_empty() && !title.is_empty() && !details.is_empty()
        && project_id != 0 && folder_id != 0);

    if !name.is_empty() && !title.is_empty() && !details.is_empty()
        && project_id != 0 && folder_id != 0 {
        let repo = get_image_repository();

        let img = raster::open(filename.as_str()).unwrap();

        let image: Image = Image {
            id: 0,
            name,
            title,
            created_by: 0,
            modified_by: 0,
            created_on: Utc::now(),
            modified_on: Utc::now(),
            project_id,
            folder_id,
            encoding: Encoding::JPG,
            height: img.height as u16,
            width: img.width as u16,
            is_published: false,
        };

        let res = repo.add(image);

        if res {
            println!("Image successfully added!");
        } else {
            println!("Image could not be added due to an error!");
        }
    }

    // TODO: assign `false` to save_image when something is not right!
    if save_image && !filename.clone().is_empty() {
        let new_filename = format!("image-uploads/{}.jpg", Uuid::new_v4());

        println!("Renaming from: {} to: {}", filename, new_filename);
        match rename(filename, new_filename) {
            Ok (_) => {
                return HttpResponse::Ok().content_type(ContentType::json())
                    .body("true");
            }

            Err(_) => {
                return HttpResponse::Ok().content_type(ContentType::json())
                    .body("false");
            }
        }
    } else if !filename.clone().is_empty() {
        // Remove the temp file.
        match remove_file(filename) {
            Ok (_) => {
                return HttpResponse::Ok().content_type(ContentType::json())
                    .body("true");
            }

            Err(_) => {
                return HttpResponse::Ok().content_type(ContentType::json())
                    .body("false");
            }
        }
    }

    return HttpResponse::Ok().content_type(ContentType::json()).body("true");
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
