///
/// Module to download file.
///
/// Release under MIT License.
///
use std::process::Command;

pub trait DownloadHelper {
    /// Run command and return true if success.
    fn download(&self, url: &str, output_filename: &str) -> bool;
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
}

#[cfg(test)]
pub mod tests {
    use super::DownloadHelper;
    use std::cell::RefCell;
    use io::InputOutputHelper;

    /// When run a container
    pub struct TestDownload {
        pub url: String,
        pub output_filename: String
    }

    pub struct TestDownloadHelper<'a> {
        io_helper: &'a InputOutputHelper,
        pub cmds:  RefCell<Vec<TestDownload>>
    }

    impl<'a> DownloadHelper for TestDownloadHelper<'a> {
        fn download(&self, url: &str, output_filename: &str) -> bool {
            let c = TestDownload {
              url: String::from(url),
              output_filename: String::from(output_filename)
            };

            self.cmds.borrow_mut().push(c);

            match self.io_helper.file_write(output_filename, url) {
                Ok(_) => true,
                Err(_) => false
            }
        }
    }

    impl<'a> TestDownloadHelper<'a> {
        pub fn new(io_helper: &InputOutputHelper) -> TestDownloadHelper {
            TestDownloadHelper {
                io_helper: io_helper,
                cmds: RefCell::new(Vec::new())
            }
        }
    }
}
