///
/// Module to print output.
///
/// Release under MIT License.
///
use std::io::{Error, ErrorKind, Write, stdin, stdout, Read};
use std::path::Path;
use std::fs::{File, remove_dir_all, create_dir_all, write, hard_link, copy};
use glob::glob;
use dirs::home_dir;

#[cfg(test)]
pub mod tests;


/// Convert path with start "~/"
pub fn convert_path(x: &str) -> String {
    if x.starts_with("~/") {
        if let Some(path) = home_dir() {
            return path.to_str().unwrap().to_owned() + &x[1..]
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
pub struct DefaultInputOutputHelper;

impl InputOutputHelper for DefaultInputOutputHelper {
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
            Ok(_) => {
                input
            }
            Err(error) => panic!("error: {}", error)
        }
    }

    fn file_write(&self, path: &str, contents: &str) -> Result<(), Error> {
        match write(path, contents) {
            Ok(f) => Ok(f),
            Err(e) => Err(e)
        }
    }

    fn file_exits(&self, filename: &str) -> bool {
        Path::new(filename).exists()
    }

    fn file_read_at_string(&self, filename: &str) -> Result<String, Error> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn dir_list_file(&self, dir: &str, pattern: &str) -> Result<Vec<String>, Error> {
        let mut new_dir = String::from(convert_path(dir));

        if ! dir.ends_with("/") {
            new_dir.push_str("/");
        }

        new_dir.push_str(pattern);

        match glob(&new_dir) {
            Ok(all_files) => {
                let mut result: Vec<String> = Vec::new();

                for entry in all_files {
                    if let Ok(path) = entry {
                        result.push(path.display().to_string());
                    }
                }

                Ok(result)
            },
            Err(e) => Err(Error::new(ErrorKind::PermissionDenied, e.msg))
        }

    }

    fn create_dir_all(&self, dir: &str) -> Result<(), Error> {
        match create_dir_all(dir) {
            Ok(a) => Ok(a),
            Err(_) => Err(Error::new(ErrorKind::PermissionDenied, "Cannot write"))
        }
    }

    fn remove_dir_all(&self, dir: &str) -> Result<(), Error> {
        match remove_dir_all(dir) {
            Ok(a) => Ok(a),
            Err(_) => Err(Error::new(ErrorKind::PermissionDenied, "Cannot delete"))
        }
    }

    fn hardlink_or_copy_file(&self, from: &str, to: &str) -> Result<(), Error> {
        match hard_link(from, to) {
            Ok(_) => Ok(()),
            Err(_) => {
                match copy(from, to) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Error::new(ErrorKind::PermissionDenied, "Cannot write"))
                }
            }
        }
    }
}
