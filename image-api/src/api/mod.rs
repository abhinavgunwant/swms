pub mod image;
pub mod admin;
pub mod service;

use log::info;
use actix_web::{get, HttpResponse, Responder, web::{ ServiceConfig, scope } };

use crate::{
    APP_NAME, APP_VERSION, auth::AuthMiddleware,
    api::admin::auth::{ auth, auth_logout, auth_refresh },
};

static mut COUNTER: u128 = 0;
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
    let c: u128;

    unsafe {
        COUNTER += 1;
        c = COUNTER;
    }

    info!("Echo request number: {}", c);

    HttpResponse::Ok().body(
        format!("{} v{}\n\nReq Counter: {}", APP_NAME, APP_VERSION, c)
    )
}

#[get("/api/am-i-logged-in")]
async fn am_i_logged_in(_: AuthMiddleware) -> HttpResponse {
    HttpResponse::Ok().body("You're signed in!")
}

