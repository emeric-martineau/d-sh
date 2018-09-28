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
use std::env::home_dir;

/// Trait to write one screen.
pub trait InputOutputHelper {
    /// Print a line with line-feed.
    fn println(&mut self, expr: &str);
    /// Print a string.
    fn print(&mut self, expr: &str);
    /// Print a line with line-feed on stderr.
    fn eprintln(&mut self, expr: &str);
    /// Read a line
    fn read_line(&mut self) -> String;
    /// Write a file.
    fn file_write(&mut self, path: &str, contents: &str) -> Result<(), Error>;
    /// Check if file exits
    fn file_exits(&mut self, filename: &str) -> bool;
    /// Read file and return as string
    fn file_read_at_string(&mut self, filename: &str) -> Result<String, Error>;
    /// List file in folder
    fn dir_list_file(&mut self, dir: &str, pattern: &str) -> Result<Vec<String>, Error>;
    /// Create all dir
    fn create_dir_all(&mut self, dir: &str) -> Result<(), Error>;
}

/// Default print on tty.
pub struct DefaultInputOutputHelper;

impl InputOutputHelper for DefaultInputOutputHelper {
    fn println(&mut self, expr: &str) {
        println!("{}", expr);
    }

    fn print(&mut self, expr: &str) {
        print!("{}", expr);
        io::stdout().flush().unwrap();
    }

    fn eprintln(&mut self, expr: &str) {
        eprintln!("{}", expr);
    }

    fn read_line(&mut self) -> String {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input
            }
            Err(error) => panic!("error: {}", error)
        }
    }

    fn file_write(&mut self, path: &str, contents: &str) -> Result<(), Error> {
        match fs::write(path, contents) {
            Ok(f) => Ok(f),
            Err(e) => Err(e)
        }
    }

    fn file_exits(&mut self, filename: &str) -> bool {
        Path::new(filename).exists()
    }

    fn file_read_at_string(&mut self, filename: &str) -> Result<String, Error> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn dir_list_file(&mut self, dir: &str, pattern: &str) -> Result<Vec<String>, Error> {
        let mut new_dir = String::from(dir);

        if ! dir.ends_with("/") {
            new_dir.push_str("/");
        }

        if dir.starts_with("~/") {
            if let Some(path) = home_dir() {
                new_dir = path.to_str().unwrap().to_owned() + &new_dir[1..];
            }
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

    fn create_dir_all(&mut self, dir: &str) -> Result<(), Error> {
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

    /// Use this fonction for test.
    pub struct TestInputOutputHelper {
        pub stdout: Vec<String>,
        pub stderr: Vec<String>,
        pub stdin: Vec<String>,
        pub files: HashMap<String, String>,
        pub files_error: HashMap<String, bool>
    }

    impl InputOutputHelper for TestInputOutputHelper {
        fn println(&mut self, expr: &str) {
            self.stdout.push(String::from(expr));
        }

        fn print(&mut self, expr: &str) {
            self.stdout.push(String::from(expr));
        }

        fn eprintln(&mut self, expr: &str) {
            self.stderr.push(String::from(expr));
        }

        fn read_line(&mut self) -> String {
            self.stdin.remove(0)
        }

        fn file_write(&mut self, path: &str, contents: &str) -> Result<(), Error> {
            if self.files_error.contains_key(path) {
                Err(Error::new(ErrorKind::PermissionDenied, "Cannot write"))
            } else {
                self.files.insert(String::from(path), String::from(contents));
                Ok(())
            }
        }

        fn file_exits(&mut self, filename: &str) -> bool {
            self.files.contains_key(filename)
        }

        fn file_read_at_string(&mut self, filename: &str) -> Result<String, Error> {
            match self.files.get(filename) {
                Some(data) => Ok(data.to_string()),
                None => Err(Error::new(ErrorKind::NotFound, "Not found"))
            }
        }

        fn dir_list_file(&mut self, dir: &str, pattern: &str) -> Result<Vec<String>, Error> {
            let regex = pattern.replace(r".", r"\.").replace(r"*", r".*");

            let re = Regex::new(&regex).unwrap();

            let file_in_folder = self.files
                .keys()
                .filter(|k| k.starts_with(dir) && re.is_match(k))
                // Convert &str to String
                .map(|k| k.to_string())
                .collect();

            Ok(file_in_folder)
//            Err(Error::new(ErrorKind::NotFound, "Not found"))
        }

        fn create_dir_all(&mut self, dir: &str) -> Result<(), Error> {
            if self.files_error.contains_key(dir) {
                Err(Error::new(ErrorKind::PermissionDenied, "Cannot write"))
            } else {
                Ok(())
            }
        }
    }

    impl TestInputOutputHelper {
        pub fn new() -> TestInputOutputHelper {
            TestInputOutputHelper {
                stdout: Vec::new(),
                stderr: Vec::new(),
                stdin: Vec::new(),
                files: HashMap::new(),
                files_error: HashMap::new()
            }
        }
    }
}
