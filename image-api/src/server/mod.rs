pub mod config;
pub mod db;

use std::sync::Arc;
use actix_web::{
    App, HttpServer, web::{ PayloadConfig, Data }, middleware::Logger,
};
use actix_cors::Cors;
use actix_web_static_files::ResourceFiles;

use crate::{
    server_state::ServerState, repository::Repository, api, generate
};
use config::ServerConfig;

#[actix_web::main]
pub async fn run_server(
    repo_arc: Arc<dyn Repository + Sync + Send>,
    server_config: ServerConfig,
) -> std::io::Result<()> {
    // let server_config = ServerConfig::default();
    // server_config.print_info();
    let srv_config = server_config.clone();

    let server_state_data = Data::new(ServerState::default());

    HttpServer::new(move || {
        let repository_data: Data<dyn Repository + Sync + Send> = Data::from(
            repo_arc.clone()
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
            .app_data(Data::new(srv_config.clone()))
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
            .service(api::admin::project::get_project)
            .service(api::admin::project::get_projects)
            .service(api::admin::project::add_project)
            .service(api::admin::project::get_projects_for_user)
            .service(api::admin::project::remove_project)
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
    .bind((server_config.get_hostname(), server_config.get_port()))?
    .run()
    .await
}

