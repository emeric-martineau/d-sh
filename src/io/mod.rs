///
/// Module to print output.
///
/// Release under MIT License.
///
use std::io;
use std::io::{Error, ErrorKind};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use glob::glob;
use std::fs::create_dir_all;
use dirs::home_dir;

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
}

/// Default print on tty.
pub struct DefaultInputOutputHelper;

impl InputOutputHelper for DefaultInputOutputHelper {
    fn println(&self, expr: &str) {
        println!("{}", expr);
    }

    fn print(&self, expr: &str) {
        print!("{}", expr);
        io::stdout().flush().unwrap();
    }

    fn eprintln(&self, expr: &str) {
        eprintln!("{}", expr);
    }

    fn read_line(&self) -> String {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input
            }
            Err(error) => panic!("error: {}", error)
        }
    }

    fn file_write(&self, path: &str, contents: &str) -> Result<(), Error> {
        match fs::write(path, contents) {
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
}

#[cfg(test)]
pub mod tests {
    use super::InputOutputHelper;
    use std::collections::HashMap;
    use std::io::{Error, ErrorKind};
    use regex::Regex;
    use std::cell::RefCell;

    pub fn found_item(list: &Vec<String>, value: &str) {
        let result: Vec<String> = list
            .iter()
            .filter(|l| *l == value)
            .map(|l| l.to_string())
            .collect();

        assert_eq!(result.len(), 1, "Cannot find '{}' value in list", value);
    }

    /// Use this fonction for test.
    pub struct TestInputOutputHelper {
        pub stdout: RefCell<Vec<String>>,
        pub stderr: RefCell<Vec<String>>,
        pub stdin: RefCell<Vec<String>>,
        pub files: RefCell<HashMap<String, String>>,
        pub files_error: RefCell<HashMap<String, bool>>,
        pub files_delete: RefCell<HashMap<String, String>>,
    }

    impl InputOutputHelper for TestInputOutputHelper {
        fn println(&self, expr: &str) {
            self.stdout.borrow_mut().push(String::from(expr));
        }

        fn print(&self, expr: &str) {
            self.stdout.borrow_mut().push(String::from(expr));
        }

        fn eprintln(&self, expr: &str) {
            self.stderr.borrow_mut().push(String::from(expr));
        }

        fn read_line(&self) -> String {
            self.stdin.borrow_mut().remove(0)
        }

        fn file_write(&self, path: &str, contents: &str) -> Result<(), Error> {
            if self.files_error.borrow().contains_key(path) {
                Err(Error::new(ErrorKind::PermissionDenied, "Cannot write"))
            } else {
                self.files.borrow_mut().insert(String::from(path), String::from(contents));
                Ok(())
            }
        }

        fn file_exits(&self, filename: &str) -> bool {
            self.files.borrow().contains_key(filename)
        }

        fn file_read_at_string(&self, filename: &str) -> Result<String, Error> {
            match self.files.borrow().get(filename) {
                Some(data) => Ok(data.to_string()),
                None => Err(Error::new(ErrorKind::NotFound, "Not found"))
            }
        }

        fn dir_list_file(&self, dir: &str, pattern: &str) -> Result<Vec<String>, Error> {
            if self.files_error.borrow().contains_key(dir) {
                Err(Error::new(ErrorKind::PermissionDenied, "Cannot read"))
            } else {
                let regex = pattern.replace(r".", r"\.").replace(r"*", r".*");

                let re = Regex::new(&regex).unwrap();

                let file_in_folder = self.files.borrow()
                    .keys()
                    .filter(|k| k.starts_with(dir) && re.is_match(k))
                    // Convert &str to String
                    .map(|k| k.to_string())
                    .collect();

                Ok(file_in_folder)
            }
//            Err(Error::new(ErrorKind::NotFound, "Not found"))
        }

        fn create_dir_all(&self, dir: &str) -> Result<(), Error> {
            if self.files_error.borrow().contains_key(dir) {
                Err(Error::new(ErrorKind::PermissionDenied, "Cannot write"))
            } else {
                Ok(())
            }
        }
    }

    impl TestInputOutputHelper {
        pub fn new() -> TestInputOutputHelper {
            TestInputOutputHelper {
                stdout: RefCell::new(Vec::new()),
                stderr: RefCell::new(Vec::new()),
                stdin: RefCell::new(Vec::new()),
                files: RefCell::new(HashMap::new()),
                files_error: RefCell::new(HashMap::new()),
                files_delete: RefCell::new(HashMap::new())
            }
        }
    }
}
