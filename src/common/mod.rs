///
/// Module of config file.
///
/// Release under MIT License.
///
extern crate serde_yaml;

use std::env::home_dir;
use std::io::{Error, ErrorKind};
use super::io::InputOutputHelper;

/// Config structure of D-SH
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub download_dir: String,
    pub applications_dir: String
}

/// Config structure of D-SH
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigApplication {
    pub image_name: String
}

/// Default config filename.
pub const DEFAULT_CONFIG_FILE: &str = ".d-sh/config.yml";

///
/// Function to return config filename.
///
pub fn get_config_filename() -> Option<String> {
    match home_dir() {
        Some(path) => {
            let home_dir = match path.to_str() {
                None => String::from(""),
                Some(p) => {
                    let mut result = String::from(p);

                    if ! p.ends_with("/") {
                        result.push_str("/");
                    }

                    result
                }
            };

            let mut config_file = String::from(home_dir);
            config_file.push_str(DEFAULT_CONFIG_FILE);

            Some(config_file)
        },
        None => None
    }
}

///
/// Return config structure.
///
pub fn get_config(io_helper: &mut InputOutputHelper) -> Result<Config, Error> {
    match get_config_filename() {
        Some(config_file) => {
            let data = io_helper.file_read_at_string(&config_file)?;
            // let deserialized_config: Config = serde_yaml::from_str(&data).unwrap();
            //
            // Ok(deserialized_config)

            match serde_yaml::from_str(&data) {
                Ok(deserialized_config) => Ok(deserialized_config),
                Err(_) => Err(Error::new(ErrorKind::Other, "File format of config file is wrong !"))
            }
        },
        None => Err(Error::new(ErrorKind::PermissionDenied, "Cannot read config file !"))
    }
}

///
/// Return config application structure.
///
pub fn get_config_application(io_helper: &mut InputOutputHelper, filename: &str) -> Result<ConfigApplication, Error> {
    let data = io_helper.file_read_at_string(&filename)?;

    match serde_yaml::from_str(&data) {
        Ok(deserialized_config) => Ok(deserialized_config),
        Err(_) => Err(Error::new(ErrorKind::Other, "File format of config application file is wrong !"))
    }
}
