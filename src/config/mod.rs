///
/// Module of config file.
///
/// Release under MIT License.
///
extern crate serde_yaml;

pub mod dockerfile;

use dirs::home_dir;
use io::convert_path;
use io::InputOutputHelper;
use std::io::{Error, ErrorKind};
use std::path::Path;

/// Config structure of D-SH
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfigDocker {
    pub from: String,
    pub tag: String,
}

/// Config structure of D-SH
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub download_dir: String,
    pub applications_dir: String,
    pub dockerfile: ConfigDocker,
    pub dockerfile_filename: Option<String>,
    pub entrypoint_filename: Option<String>,
    pub tmp_dir: Option<String>,
}

/// Config structure of D-SH
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigApplication {
    pub image_name: String,
    pub cmd_line: String,
    pub cmd_line_args: Option<Vec<String>>,
    pub interactive: Option<bool>,
    pub ipc_host: Option<bool>,
    pub dependencies: Option<Vec<String>>,
    pub download_filename: String,
    pub url: Option<String>,
    pub skip_redownload: Option<bool>,
}

/// Default config filename.
pub const DEFAULT_CONFIG_FILE_PATH: &str = ".d-sh/";
pub const DEFAULT_CONFIG_FILE: &str = "config.yml";

///
/// Function to return config filename.
///
pub fn get_config_filename() -> Option<String> {
    create_config_filename_path(DEFAULT_CONFIG_FILE)
}

///
/// Function to create a path for file.
///
pub fn create_config_filename_path(filename: &str) -> Option<String> {
    match home_dir() {
        Some(path) => {
            let home_dir = match path.to_str() {
                None => String::from(""),
                Some(p) => {
                    let mut result = String::from(p);

                    if !p.ends_with("/") {
                        result.push_str("/");
                    }

                    result
                }
            };

            let mut config_file = String::from(home_dir);
            config_file.push_str(DEFAULT_CONFIG_FILE_PATH);
            config_file.push_str(filename);

            Some(config_file)
        }
        None => None,
    }
}

///
/// Return config structure.
///
pub fn get_config(config_filename: String, io_helper: &InputOutputHelper) -> Result<Config, Error> {
    let data = io_helper.file_read_at_string(&config_filename)?;
    // let deserialized_config: Config = serde_yaml::from_str(&data).unwrap();
    //
    // Ok(deserialized_config)

    match serde_yaml::from_str(&data) {
        Ok(deserialized_config) => Ok(deserialized_config),
        Err(err) => Err(Error::new(
            ErrorKind::Other,
            format!("File format of config file is wrong, {}!", err),
        )),
    }
}

///
/// Return config application structure.
///
pub fn get_config_application(
    io_helper: &InputOutputHelper,
    filename: &str,
) -> Result<ConfigApplication, Error> {
    let new_filename = convert_path(&filename);

    let data = io_helper.file_read_at_string(&new_filename)?;

    match serde_yaml::from_str(&data) {
        Ok(deserialized_config) => Ok(deserialized_config),
        Err(err) => Err(Error::new(
            ErrorKind::Other,
            format!("File format of config application file is wrong, {}!", err),
        )),
    }
}

///
/// Return file with dir.
///
pub fn get_filename(dir: &str, app: &str, ext: Option<&str>) -> String {
    let mut application_filename = String::from(app);

    if ext.is_some() {
        application_filename.push_str(ext.unwrap());
    }

    let application_filename_path = Path::new(dir).join(&application_filename);

    let application_filename_full_path = application_filename_path.to_str().unwrap();

    String::from(application_filename_full_path)
}
