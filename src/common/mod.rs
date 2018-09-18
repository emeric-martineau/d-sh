///
/// Module to store some common data.
///
/// Release under MIT License.
///
use std::env::home_dir;

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
