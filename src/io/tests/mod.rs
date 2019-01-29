///
/// Module to provide structure for tests.
///
/// Release under MIT License.
///
use io::InputOutputHelper;
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

    fn remove_dir_all(&self, dir: &str) -> Result<(), Error> {
        if self.files_error.borrow().contains_key(dir) {
            Err(Error::new(ErrorKind::PermissionDenied, "Cannot delete"))
        } else {
            // Collect file to delete
            let files_to_delete: Vec<String> = self.files.borrow()
                .keys()
                .filter(|k| k.starts_with(dir))
                // Convert &str to String
                .map(|k| k.to_string())
                .collect();

            let mut deleted_files = self.files_delete.borrow_mut();
            let mut files = self.files.borrow_mut();

            // Move file from `files` to `files_delete`
            for filename in &files_to_delete {
                deleted_files.insert(filename.to_owned(),
                        files.get(filename).unwrap().to_owned());
                files.remove(filename);
            }

            Ok(())
        }
    }

    fn hardlink_or_copy_file(&self, from: &str, to: &str) -> Result<(), Error> {
        match self.file_read_at_string(&from) {
            Ok(content) => {
                match self.file_write(&to, &content) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err)
                }
            },
            Err(err) => Err(err)
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
