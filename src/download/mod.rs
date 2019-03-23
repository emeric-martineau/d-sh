///
/// Module to download file.
///
/// Release under MIT License.
///
use std::process::Command;
use log::LoggerHelper;

#[cfg(test)]
pub mod tests;

pub trait DownloadHelper {
    /// Download a file.
    fn download(&self, url: &str, output_filename: &str) -> bool;
    /// Download file if updated. Check date of file.
    fn download_if_update(&self, url: &str, output_filename: &str) -> bool;
}

/// Default run process
pub struct DefaultDownloadHelper<'a> {
    log_helper: &'a LoggerHelper
}

impl<'a> DefaultDownloadHelper<'a> {
    pub fn new(log_helper: &LoggerHelper) -> DefaultDownloadHelper {
        DefaultDownloadHelper {
            log_helper
        }
    }
}

impl<'a> DownloadHelper for DefaultDownloadHelper<'a> {
    fn download(&self, url: &str, output_filename: &str) -> bool {
        let args = &["-o", output_filename, "-L", url];

        self.log_helper.debug_with_array("Run command 'curl{}'", args.into_iter());

        match Command::new("curl")
            .args(args)
            .status()
        {
            Ok(s) => {
                if let Some(exit_code) = s.code() {
                    self.log_helper.debug_with_parameter("Curl return exit code: {}", &exit_code.to_string());
                }

                true
            },
            Err(e) => {
                self.log_helper.err(&e.to_string());
                false
            },
        }
    }

    fn download_if_update(&self, url: &str, output_filename: &str) -> bool {
        let args = &["-o", output_filename, "-z", output_filename, "-L", url];

        self.log_helper.debug_with_array("Run command 'curl{}'", args.into_iter());

        match Command::new("curl")
            .args(args)
            .status()
        {
            Ok(s) => {
                if let Some(exit_code) = s.code() {
                    self.log_helper.debug_with_parameter("Curl return exit code: {}", &exit_code.to_string());
                }

                true
            },
            Err(e) => {
                self.log_helper.err(&e.to_string());
                false
            },
        }
    }
}
