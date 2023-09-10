pub mod image;
pub mod admin;
pub mod service;

use actix_web::{get, HttpResponse, Responder};

pub const DEST_REN_DIR: &str = "image-rendition-cache";
pub const IMG_UPL_DIR: &str = "image-uploads";

#[get("/api/echo")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Image API, Version 0.0.1\n\nAPI is live!")
}

