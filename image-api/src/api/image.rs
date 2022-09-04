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
use crate::repository;
use crate::repository::{ Repository, item::Item };
use crate::repository::image::{ Image, ImageItem, encoding::Encoding };

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
    let image_file = web::block(
            move || read(format!("image-uploads/{}", file_req.filename))
        )
        .await
        .unwrap()
        .expect("Error while downloading");

    return HttpResponse::build(StatusCode::OK)
        .content_type("image/jpeg")
        .body(image_file);
}

#[get("/api/imagedata")]
pub async fn imagedata() -> web::Json<ImageJson> {
    let repo = repository::get_image_repository();

    //let repo: dyn Repository = boxed_repo.downcast().expect("Problem unboxing repository");
//    let repo: dyn Repository = *boxed_repo;

    let image = repo.get(0);
    //let imageItem = image as Box<dyn ImageItem>;

    let img: ImageJson = ImageJson {
        id: image.id(),
        slug: image.slug(),
    };

    web::Json(img)
}

