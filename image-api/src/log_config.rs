use std::path::Path;

use log::warn;
use log4rs::init_file;

const DEFAULT_LOG_FILE: &str = "log4rs.yml";

pub fn init_logger() {
    if Path::new(DEFAULT_LOG_FILE).exists() {
        init_file(DEFAULT_LOG_FILE, Default::default()).unwrap();
    } else {
        warn!("No logger config found. Running logger with default config.");
    }
}

