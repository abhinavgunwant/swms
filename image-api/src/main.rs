mod api;
mod db;
mod repository;
mod auth;
mod model;

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
            .service(api::admin::get_children)
            .service(api::admin::auth::auth)
            .service(api::admin::auth::get_user_permissions)
            .service(api::admin::user::create_user)
            .service(api::admin::user::edit_user)
            .service(api::admin::user::get_user)
            .service(api::admin::user::get_user_list)
            .service(api::admin::user::search_user)
            .service(api::admin::role::get_all_roles)
            .service(api::admin::role::set_role)
            .service(api::admin::role::update_role)
            .service(api::admin::role::delete_role)
            .service(api::admin::project::get_projects)
            .service(api::admin::project::add_project)
            .service(api::admin::project::get_projects_for_user)
            .service(api::admin::project::validate_slug)
            .service(api::admin::image::get_images_in_project)
            .service(api::admin::image::get_image)
            .service(api::admin::image::get_image_file)
            .service(api::admin::image::add_image)
            .service(api::admin::image::remove_image)
            .service(api::admin::image::update_image_title)
            .service(api::admin::folder::get_folder)
            .service(api::admin::folder::add_folder)
            .service(api::admin::folder::update_folder)
            .service(api::admin::folder::remove_folder)
            .service(api::admin::rendition::get_renditions_for_image)
            .service(api::admin::rendition::get_rendition)
            .service(api::admin::rendition::set_rendition)
            .service(api::image::upload)
            .service(api::image::download)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
