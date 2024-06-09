mod api;
mod db;
mod repository;
mod auth;
mod model;
mod server_state;
mod log_config;
mod server;
mod error;

use std::{ env, sync::Arc };

use error::SWMSError;
use log::{ info, error };
use server::config::ServerConfig;

use server::run_server;
use repository::{ get_repository, Repository, RepositoryStartupError };

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const LINE: &str = "\n--------------------";

fn main() -> Result<(), SWMSError> {
    log_config::init_logger();

    info!("{}\n {} v{}{}", LINE, APP_NAME, APP_VERSION, LINE);
    info!("Starting Up");

    let server_config = ServerConfig::default();
    server_config.print_info();

    let repository_arc: Arc<dyn Repository + Sync + Send>;

    // get repository or exit on error.
    match get_repository() {
        Ok(repo_arc) => { repository_arc = repo_arc; }

        Err(repo_err) => {
            match repo_err {
                RepositoryStartupError::ConnectionError => {
                    return Err(SWMSError::repository());
                }

                // First time startup, probably!
                RepositoryStartupError::DatabaseDoesNotExist => {
                    return Err(SWMSError::repository());
                }

                _ => { return Err(SWMSError::repository()); }
            }
        }
    }

    // start the actix server
    match run_server(repository_arc, server_config) {
        Ok(_) => Ok(()),

        Err(e) => {
            error!("Error: {}", e);
            return Err(SWMSError::actix_server());
        }
    }
}

