mod api;
mod db;
mod repository;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use actix_web_static_files::ResourceFiles;
use actix_form_data::{Field, Form};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // db::adapters::mysql::print_data();

    HttpServer::new(|| {
        let generated = generate();
        App::new()
            .app_data(web::PayloadConfig::new(1000000 * 250))
            .service(api::echo)
            .service(api::image::upload)
            .service(api::image::download)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
