pub mod image;
pub mod admin;
pub mod service;

use actix_web::{get, HttpResponse, Responder, web::{ ServiceConfig, scope } };

use crate::api::admin::auth::{ auth, auth_logout, auth_refresh };

pub const DEST_REN_DIR: &str = "image-rendition-cache";
pub const IMG_UPL_DIR: &str = "image-uploads";

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api/admin/auth")
        .service(auth)
        .service(auth_logout)
        .service(auth_refresh);

    conf.service(scope);
}

#[get("/api/echo")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Image API, Version 0.0.1\n\nAPI is live!")
}

#[get("/api/am-i-logged-in")]
async fn am_i_logged_in() -> HttpResponse {
    HttpResponse::Ok().body("You're signed in!")
}

