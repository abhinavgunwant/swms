pub mod image;
pub mod admin;

use actix_web::{get, HttpResponse, Responder};

#[get("/api/echo")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Image API, Version 0.0.1\n\nAPI is live!")
}
