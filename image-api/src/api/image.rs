// use std::path::PathBuf;
use std::io::Write;
use std::fs::{ File, create_dir_all, read };
use std::path::Path;
use actix_multipart::Multipart;
use actix_web::{ get, post, web, HttpResponse, Responder, http::{StatusCode} };
use actix_form_data::{ handle_multipart, Error, Field, Form, Value };
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
use serde::{ Serialize, Deserialize };
use raster;
use crate::repository;
use crate::repository::{ Repository, item::Item };
use crate::repository::image::{ Image, encoding::Encoding, ImageRepository, get_image_repository };

#[derive(Serialize)]
pub struct ImageJson {
    slug: String,
    id: u32,
}

#[derive(Debug, Deserialize)]
pub struct FileRequest {
    filename: String
}

#[post("/api/image")]
pub async fn upload(mut payload: Multipart, file_path: String) -> HttpResponse {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        // let content_type = field.content_disposition().unwrap();
        //let filename = content_type.get_filename().unwrap();
        // let filepath = format!(".{}", file_path);

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

#[get("/api/image")]
pub async fn download(file_req: web::Query<FileRequest>) -> HttpResponse {
    let tmp = Vec::from_iter(file_req.filename.split(".").map(String::from));

    let source_file_path = format!("image-uploads/{}.{}", tmp[0], tmp[1]);

    // Here `-mobile` describes the name of the rendition (not meant to be in final code).
    let dest_file_name = format!(
        "image-rendition-cache/{}-mobile.{}",
        tmp[0],
        tmp[1]
    );

    let mut image = raster::open(source_file_path.as_str()).unwrap();

    // Assuming this rendition to be intended for mobile with width 480,
    // calculate height (ofcourse with final product, this will be
    // fully configurable).
    let new_height: i32 = 480*&image.height/&image.width;

    raster::editor::resize(&mut image, 480, new_height, raster::ResizeMode::Fit).unwrap();
    raster::save(&image, dest_file_name.as_str()).unwrap();

    let image_file = web::block(
        move || read(String::from(dest_file_name))
        ).await.unwrap().expect("Error whie downloading!");

    return HttpResponse::build(StatusCode::OK)
        .content_type("image/jpeg")
        .body(image_file);
        //.body(image.bytes);
}

#[get("/api/imagedata")]
pub async fn imagedata() -> web::Json<Image> {
    let repo = get_image_repository();

    println!("got id: {}, name: {}", repo.get(0).id, repo.get(0).name);

    //let repo: dyn Repository = boxed_repo.downcast().expect("Problem unboxing repository");
//    let repo: dyn Repository = *boxed_repo;

    let image = repo.get(0);
    //let imageItem = image as Box<dyn ImageItem>;

    let img: ImageJson = ImageJson {
        id: image.id,
        slug: image.slug.clone(),
    };

    web::Json(image)
}

