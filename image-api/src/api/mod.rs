pub mod image;
pub mod admin;
pub mod service;

use actix_web::{get, HttpResponse, Responder, web::{ ServiceConfig, scope } };

use crate::api::admin::auth::{ auth, auth_logout };

pub const DEST_REN_DIR: &str = "image-rendition-cache";
pub const IMG_UPL_DIR: &str = "image-uploads";

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api/admin/auth")
        .service(auth)
        .service(auth_logout);

    conf.service(scope);
}

#[get("/api/echo")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Image API, Version 0.0.1\n\nAPI is live!")
}

