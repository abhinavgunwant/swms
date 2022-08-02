use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

static mut COUNTER: u128 = 0;

#[get("/api/echo")]
async fn echo() -> impl Responder {
    unsafe {
        COUNTER += 1;
        HttpResponse::Ok().body(format!("Image API version 0.0.1\n\n{}", COUNTER))
    }
}

#[get("/api/echo/mysql")]
async fn echoMySQL() -> impl Responder {
    
}
