pub mod image;
pub mod admin;
pub mod service;

use log::info;
use actix_web::{ get, HttpResponse, Responder };

use crate::{ APP_NAME, APP_VERSION, auth::AuthMiddleware };

static mut COUNTER: u128 = 0;

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

fn resp_400() -> HttpResponse {
    HttpResponse::BadRequest().body("Bad Request")
}

fn resp_404() -> HttpResponse {
    HttpResponse::NotFound().body("Not Found")
}

fn resp_500() -> HttpResponse {
    HttpResponse::InternalServerError().body("Internal Server Error")
}

