mod api;
mod db;
mod repository;
mod auth;

use actix_web::{ App, HttpServer, web };
use actix_cors::Cors;
use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // TODO: Implement a stricter CORS policy.
        let cors = //Cors::default()
            Cors::permissive();
            // .allowed_origin("https://localhost:3000/")
            // .allowed_origin("https://localhost:8080/")
            // .allowed_methods(vec![
            //     "GET", "POST", "PUT", "HEAD", "OPTIONS",
            // ])
            // .allowed_headers(vec![
            //     header::AUTHORIZATION,
            //     header::ACCEPT,
            //     header::CONTENT_TYPE,
            //     header::USER_AGENT,
            //     header::
            // ])
            // .max_age(3600);
        let generated = generate();
        App::new()
            .wrap(cors)
            .app_data(web::PayloadConfig::new(1000000 * 250))
            .service(api::echo)
            .service(api::admin::auth::auth)
            .service(api::admin::user::create_user)
            .service(api::admin::user::get_user)
            .service(api::admin::project::get_projects)
            .service(api::admin::project::add_project)
            .service(api::admin::project::get_projects_for_user)
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
