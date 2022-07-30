use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/image")]
async fn image() -> impl Responder {
    HttpResponse::Ok().body("This API shall return a processed image")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let generated = generate();
        App::new()
            .service(hello)
            .service(image)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
