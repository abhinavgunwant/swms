pub mod image;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

static mut COUNTER: u128 = 0;

#[get("/api/echo")]
async fn echo() -> impl Responder {
    COUNTER += 1;
    HttpResponse::Ok().body(format!("Image API version 0.0.1\n\n{}", COUNTER))
}
