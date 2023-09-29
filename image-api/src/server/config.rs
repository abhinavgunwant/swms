use std::{
    convert::From, path::Path,
    fs::{ create_dir_all, read_to_string, write }
};

use serde::{ Serialize, Deserialize };
use serde_yaml;
use dirs_next::{ data_local_dir, config_dir };
use log::{ info, debug, error, warn };

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub hostname: String,
    pub port: u16,
    pub upload_dir: String,
    pub rendition_cache_dir: String,
}

const DEF_ROOT_DIR_NAME: &str = "dam";
const DEF_IMG_UPL_DIR_NAME: &str = "uploads";
const DEF_REN_DIR_NAME: &str = "renditions";

impl Default for ServerConfig {
    fn default() -> Self {
        let upload_dir: String;
        let rendition_cache_dir: String;
        let config_dir: String = Self::get_config_dir_path();
        let config_file: String = Self::get_config_path();

        let hostname = String::from("localhost");
        let port: u16 = 5421;

        if let Some(data_dir) = data_local_dir() {
            let dir = data_dir.display();

            upload_dir = format!(
                "{}/{}/{}", dir, DEF_ROOT_DIR_NAME, DEF_IMG_UPL_DIR_NAME
            );

            rendition_cache_dir = format!(
                "{}/{}/{}", dir, DEF_ROOT_DIR_NAME, DEF_REN_DIR_NAME
            );
        } else {
            upload_dir = String::default();
            rendition_cache_dir = String::default();
        }

        let config_file_str = config_file.as_str();

        if Path::new(config_file_str).exists() {
            match read_to_string(config_file_str) {
                Ok(file_content) => {
                    match serde_yaml::from_str(&file_content) {
                        Ok(s_conf) => { return s_conf; }
                        Err(e) => {
                            error!("An error occured while parsing config file \
                                ({}): {}", config_file, e);
                            info!("Falling back to default config.");
                        }
                    }
                }

                Err(e) => {
                    error!(
                        "Some error occured while opening file ({}): {}",
                        config_file, e
                    );
                }
            }
        } else {
            info!("No config file (config.yml) found in {}", config_dir);
            info!(
                "Creating fresh file with default configs: {}.",
                config_file
            );

            match create_dir_all(config_dir.as_str()) {
                std::io::Result::Ok(()) => {
                    debug!(
                        "Default config directory created at: {}",
                        config_dir
                    );

                    let temp_config = Self {
                        hostname: hostname.clone(),
                        port: port.clone(),
                        upload_dir: upload_dir.clone(),
                        rendition_cache_dir: rendition_cache_dir.clone()
                    };

                    match serde_yaml::to_string(&temp_config) {
                        Ok(serialized_config) => {
                            debug!("Writing config file");
                            debug!("config data: {}", serialized_config);

                            match write(config_file_str, serialized_config) {
                                std::io::Result::Ok(()) => {
                                    debug!(
                                        "config file written at: {}",
                                        config_file
                                    );
                                }

                                Err(e) => {
                                    warn!(
                                        "Could not create config directory \
                                        ({}): {}",
                                        config_dir, e
                                    );
                                    info!("Falling back to default config.");
                                }
                            }
                        }

                        Err(e) => {
                            error!(
                                "Error parsing config ({}): {}",
                                config_file, e
                            );
                            info!("Falling back to default config.");
                        }
                    }
                }

                Err(e) => {
                    warn!(
                        "Could not create config directory ({}): {}",
                        config_dir, e
                    );
                    info!("Falling back to default config.");
                }
            }
        }

        // Return default server config (no config file)

        Self { hostname, port, upload_dir, rendition_cache_dir }
    }
}

impl ServerConfig {
    pub fn print_info(&self) {
        info!("** Got the following configs:\n\
              --> upload directory: {}\n\
              --> rendition cache directory: {}\n\
              --> server host: {}\n\
              --> server port: {}",
              self.upload_dir,
              self.rendition_cache_dir,
              self.hostname,
              self.port,
        );
    }

    fn get_config_path() -> String {
        let config_dir_path = Self::get_config_dir_path();

        if !config_dir_path.is_empty() {
            return format!("{}/config.yml", config_dir_path);
        }

        String::default()
    }

    fn get_config_dir_path() -> String {
        if let Some(c_dir) = config_dir() {
            return format!("{}/{}", c_dir.display(), DEF_ROOT_DIR_NAME);
        }

        String::default()
    }
}

//impl From<ConfigBuilder<DefaultState>> for ServerConfig {
//    fn from(config_builder: ConfigBuilder<DefaultState>) -> Self {
//        let hostname: String;
//        let port: u16;
//        let upload_dir: String;
//        let rendition_cache_dir: String;
//
//        let config_builder_clone = config_builder.clone();
//
//        match config_builder_clone.build() {
//            Ok(conf) => {
//                match conf.get_string("hostname") {
//                    Ok(val) => { hostname = val; }
//
//                    Err(e) => {
//                        error!("Error while getting hostname from config: {}", e);
//
//                        hostname = String::default();
//                    }
//                }
//
//                match conf.get_int("port") {
//                    Ok(val) => { port = val as u16; }
//
//                    Err(e) => {
//                        error!("Error while getting port from config: {}", e);
//
//                        port = 0;
//                    }
//                }
//
//                match conf.get_string("uploadDir") {
//                    Ok(val) => { upload_dir = val; }
//                    Err(e) => {
//                        error!("Error while getting uploadDir from config: {}", e);
//
//                        upload_dir = String::default();
//                    }
//                }
//
//                match conf.get_string("renditionCacheDir") {
//                    Ok(val) => { rendition_cache_dir = val; }
//                    Err(e) => {
//                        error!("Error while getting renditionCacheDir from config: {}", e);
//
//                        rendition_cache_dir = String::default();
//                    }
//                }
//            }
//
//            Err(e) => {
//                error!("Error building config: {}", e);
//
//                return Self::default();
//            }
//        }
//
//        Self { hostname, port, upload_dir, rendition_cache_dir }
//    }
//}

