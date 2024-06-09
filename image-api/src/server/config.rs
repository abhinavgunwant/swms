use std::{
    convert::From, path::Path, fs::{ create_dir_all, read_to_string, write },
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
    pub db: DBConfig,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DBConfig {
    #[serde(rename = "type")]
    pub db_type: DBType,
    pub connection_info: DBConnectionInfo,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum DBType {
    #[default]
    MySQL
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DBConnectionInfo {
    pub db_name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Default for DBConnectionInfo {
    fn default() -> Self { Self {
        db_name: String::from("swms"),
        host: String::from("localhost"),
        port: 3306,
        username: String::from("swms"),
        password: String::from("swms1234"),
    } }
}

const DEF_ROOT_DIR_NAME: &str = "swms";
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
                        rendition_cache_dir: rendition_cache_dir.clone(),
                        db: DBConfig::default(),
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
        Self {
            hostname,
            port,
            upload_dir,
            rendition_cache_dir,
            db: DBConfig::default(),
        }
    }
}

impl ServerConfig {
    pub fn print_info(&self) {
        info!("Config file: {}", Self::get_config_path());

        info!("Got the following configs in config file:\n\
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

    pub fn get_connection_string(&self) -> String {
        match self.db.db_type {
            DBType::MySQL => {
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.db.connection_info.username,
                    self.db.connection_info.password,
                    self.db.connection_info.host,
                    self.db.connection_info.port,
                    self.db.connection_info.db_name,
                )
            }
        }
    }

    pub fn get_hostname(&self) -> &str {
        self.hostname.as_str()
    }

    pub fn get_port(&self) -> u16 {
        self.port.clone()
    }
}

