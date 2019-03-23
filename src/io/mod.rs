use dirs::home_dir;
use glob::glob;
use std::fs::{copy, create_dir_all, hard_link, remove_dir_all, write, File};
///
/// Module to print output.
///
/// Release under MIT License.
///
use std::io::{stdin, stdout, Error, ErrorKind, Read, Write};
use std::path::Path;
use log::LoggerHelper;

#[cfg(test)]
pub mod tests;

/// Convert path with start "~/"
pub fn convert_path(x: &str) -> String {
    if x.starts_with("~/") {
        if let Some(path) = home_dir() {
            return path.to_str().unwrap().to_owned() + &x[1..];
        }
    }

    String::from(x)
}

/// Trait to write one screen.
pub trait InputOutputHelper {
    /// Print a line with line-feed.
    fn println(&self, expr: &str);
    /// Print a string.
    fn print(&self, expr: &str);
    /// Print a line with line-feed on stderr.
    fn eprintln(&self, expr: &str);
    /// Read a line
    fn read_line(&self) -> String;
    /// Write a file.
    fn file_write(&self, path: &str, contents: &str) -> Result<(), Error>;
    /// Check if file exits
    fn file_exits(&self, filename: &str) -> bool;
    /// Read file and return as string
    fn file_read_at_string(&self, filename: &str) -> Result<String, Error>;
    /// List file in folder
    fn dir_list_file(&self, dir: &str, pattern: &str) -> Result<Vec<String>, Error>;
    /// Create all dir
    fn create_dir_all(&self, dir: &str) -> Result<(), Error>;
    /// Remove dir
    fn remove_dir_all(&self, dir: &str) -> Result<(), Error>;
    /// Create an hardlink or copy file if not possible
    fn hardlink_or_copy_file(&self, from: &str, to: &str) -> Result<(), Error>;
}

/// Default print on tty.
pub struct DefaultInputOutputHelper<'a> {
    log_helper: &'a LoggerHelper
}

impl<'a> DefaultInputOutputHelper<'a> {
    pub fn new(log_helper: &LoggerHelper) -> DefaultInputOutputHelper {
        DefaultInputOutputHelper {
            log_helper
        }
    }
}

impl<'a> InputOutputHelper for DefaultInputOutputHelper<'a> {
    fn println(&self, expr: &str) {
        println!("{}", expr);
    }

    fn print(&self, expr: &str) {
        print!("{}", expr);
        stdout().flush().unwrap();
    }

    fn eprintln(&self, expr: &str) {
        eprintln!("{}", expr);
    }

    fn read_line(&self) -> String {
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(e) => {
                self.log_helper.err(&e.to_string());
                panic!("error: {}", e)
            },
        }
    }

    fn file_write(&self, path: &str, contents: &str) -> Result<(), Error> {
        self.log_helper.debug_with_parameter("Write file '{}'", path);

        match write(path, contents) {
            Ok(f) => Ok(f),
            Err(e) => {
                self.log_helper.err(&e.to_string());
                Err(e)
            },
        }
    }

    fn file_exits(&self, filename: &str) -> bool {
        Path::new(filename).exists()
    }

    fn file_read_at_string(&self, filename: &str) -> Result<String, Error> {
        self.log_helper.debug_with_parameter("Read file '{}'", filename);

        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn dir_list_file(&self, dir: &str, pattern: &str) -> Result<Vec<String>, Error> {
        self.log_helper.debug(&format!("List files '{}' of folder '{}'", pattern, dir));

        let mut new_dir = String::from(convert_path(dir));

        if !dir.ends_with("/") {
            new_dir.push_str("/");
        }

        new_dir.push_str(pattern);

        match glob(&new_dir) {
            Ok(all_files) => {
                self.log_helper.debug("Found files:");

                let mut result: Vec<String> = Vec::new();

                for entry in all_files {
                    if let Ok(path) = entry {
                        let f = path.display().to_string();

                        self.log_helper.debug_with_parameter(" - '{}'", &f);

                        result.push(f);
                    }
                }

                Ok(result)
            }
            Err(e) => {
                self.log_helper.err(&e.to_string());
                Err(Error::new(ErrorKind::PermissionDenied, e.msg))
            },
        }
    }

    fn create_dir_all(&self, dir: &str) -> Result<(), Error> {
        self.log_helper.debug_with_parameter("Create folder '{}'", dir);

        match create_dir_all(dir) {
            Ok(a) => Ok(a),
            Err(e) => {
                self.log_helper.err(&e.to_string());
                Err(Error::new(ErrorKind::PermissionDenied, "Cannot write"))
            },
        }
    }

    fn remove_dir_all(&self, dir: &str) -> Result<(), Error> {
        self.log_helper.debug_with_parameter("Remove folder '{}'", dir);

        match remove_dir_all(dir) {
            Ok(a) => Ok(a),
            Err(e) => {
                self.log_helper.err(&e.to_string());
                Err(Error::new(ErrorKind::PermissionDenied, "Cannot delete"))
            },
        }
    }

    fn hardlink_or_copy_file(&self, from: &str, to: &str) -> Result<(), Error> {
        self.log_helper.debug(&format!("Create hard link from file '{}' to '{}", from, to));

        match hard_link(from, to) {
            Ok(_) => Ok(()),
            Err(e) => {
                self.log_helper.warn("Cannot create hard link");
                self.log_helper.warn(&e.to_string());

                match copy(from, to) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        self.log_helper.err(&e.to_string());

                        Err(Error::new(ErrorKind::PermissionDenied, "Cannot write"))
                    },
                }
            },
        }
    }
}
