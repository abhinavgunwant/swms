mod api;
mod db;
mod repository;
mod auth;
mod model;
mod server_state;
mod log_config;
mod server;

use std::{ env, sync::Arc, io::{ Error, ErrorKind } };

use actix_web::{
    App, HttpServer, web::{ PayloadConfig, Data }, middleware::Logger,
};
use actix_cors::Cors;
use actix_web_static_files::ResourceFiles;
use log::{ info, error };

use server_state::ServerState;
use server::config::ServerConfig;
use repository::{ Repository, MySQLRepository };

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const LINE: &str = "\n--------------------";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log_config::init_logger();

    info!("{}\n {} v{}{}", LINE, APP_NAME, APP_VERSION, LINE);
    info!("Starting Up");

    let server_config = ServerConfig::default();
    server_config.print_info();

    let repository_arc: Arc<dyn Repository + Sync + Send>;

    match server_config.db.db_type {
        server::config::DBType::MySQL => {
            match MySQLRepository::new(
                server_config.get_connection_string().as_str()
            ) {
                Ok(r) => {
                    repository_arc = Arc::new(r);

                    info!("Initializing MySQL repository.");
                }

                Err(e) => {
                    let error_msg = "Could not connect to MySQL database";
                    error!("{}: {}", error_msg, e);
                    return Err(Error::new(ErrorKind::InvalidData, error_msg));
                }
            }
        }
    }

    let server_state_data = Data::new(ServerState::default());

    HttpServer::new(move || {
        let repository_data: Data<dyn Repository + Sync + Send> = Data::from(
            repository_arc.clone()
        );
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
            .wrap(Logger::new("%{r}a %{Referer}i [%{User-Agent}i] %r took %D ms").log_target("request"))
            .app_data(server_state_data.clone())
            .app_data(Data::new(server_config.clone()))
            .app_data(repository_data)
            .app_data(PayloadConfig::new(1000000 * 250))
            .service(api::echo)
            .service(api::am_i_logged_in)
            .service(api::admin::get_children)
            .service(api::admin::auth::get_user_permissions)
            .service(api::admin::auth::auth)
            .service(api::admin::auth::auth_logout)
            .service(api::admin::auth::auth_refresh)
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
            .service(api::admin::image::update)
            .service(api::admin::folder::get_folder)
            .service(api::admin::folder::add_folder)
            .service(api::admin::folder::update_folder)
            .service(api::admin::folder::remove_folder)
            .service(api::admin::rendition::get_renditions_for_image)
            .service(api::admin::rendition::get_rendition)
            .service(api::admin::rendition::set_rendition)
            .service(api::admin::rendition::delete_rendition)
            .service(api::image::upload)
            .service(api::image::download)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

