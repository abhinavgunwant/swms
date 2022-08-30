mod mysql;

use crate::repository::{Repository, DBImpl};

// TODO: try moving it out to the repository directory.
pub fn get_image_repository () -> dyn Repository {
    // TODO: Read config here to get the configured DB.

    // TODO: Assign the config DB value to `db`.
    let db = DBImpl::MySQL;

    match db {
        DBImpl::MYSQL => mysql::ImageRepositoryMySQL {},
    }
}
