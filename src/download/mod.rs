///
/// Module to download file.
///
/// Release under MIT License.
///
use std::process::Command;

pub trait DownloadHelper {
    /// Download a file.
    fn download(&self, url: &str, output_filename: &str) -> bool;
    /// Download file if updated. Check date of file.
    fn download_if_update(&self, url: &str, output_filename: &str) -> bool;
}

/// Default run process
pub struct DefaultDownloadHelper;

impl DownloadHelper for DefaultDownloadHelper {
    fn download(&self, url: &str, output_filename: &str) -> bool {
        match Command::new("curl")
            .args(&["-o", output_filename, "-L", url])
            .status() {
           Ok(_) => true,
           Err(_) => false
        }
    }

    fn download_if_update(&self, url: &str, output_filename: &str) -> bool {
        match Command::new("curl")
            .args(&["-o", output_filename, "-z", output_filename, "-L", url])
            .status() {
           Ok(_) => true,
           Err(_) => false
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::DownloadHelper;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use io::InputOutputHelper;
    use io::tests::TestInputOutputHelper;

    /// When run a container
    pub struct TestDownload {
        pub url: String,
        pub output_filename: String,
        pub update: bool
    }

    pub struct TestDownloadHelper<'a> {
        io_helper: &'a TestInputOutputHelper,
        pub dl:  RefCell<Vec<TestDownload>>,
        pub update_dl_files: RefCell<HashMap<String, bool>>,
        pub urls_error: RefCell<HashMap<String, bool>>
    }

    impl<'a> DownloadHelper for TestDownloadHelper<'a> {
        fn download(&self, url: &str, output_filename: &str) -> bool {
            if self.urls_error.borrow().contains_key(url) {
                return false;
            }

            let c = TestDownload {
              url: String::from(url),
              output_filename: String::from(output_filename),
              update: true
            };

            self.dl.borrow_mut().push(c);

            match self.io_helper.file_write(output_filename, url) {
                Ok(_) => true,
                Err(_) => false
            }
        }

        fn download_if_update(&self, url: &str, output_filename: &str) -> bool {
            if ! self.update_dl_files.borrow().contains_key(output_filename) &&
                self.io_helper.files.borrow().contains_key(output_filename) {
                let c = TestDownload {
                  url: String::from(url),
                  output_filename: String::from(output_filename),
                  update: false
                };

                self.dl.borrow_mut().push(c);

                return true;
            }

            self.download(url, output_filename)
        }
    }

    impl<'a> TestDownloadHelper<'a> {
        pub fn new(io_helper: &TestInputOutputHelper) -> TestDownloadHelper {
            TestDownloadHelper {
                io_helper: io_helper,
                dl: RefCell::new(Vec::new()),
                update_dl_files: RefCell::new(HashMap::new()),
                urls_error: RefCell::new(HashMap::new())
            }
        }
    }
}
