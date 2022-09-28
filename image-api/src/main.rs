mod api;
mod db;
mod repository;
mod authtools;

use actix_web::{ App, HttpServer, web};
use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let generated = generate();
        App::new()
            .app_data(web::PayloadConfig::new(1000000 * 250))
            .service(api::echo)
            .service(api::admin::auth::auth)
            .service(api::admin::user::create_user)
            .service(api::admin::user::get_user)
            .service(api::image::upload)
            .service(api::image::download)
            .service(api::image::imagedata)
            .service(api::image::getimage)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
