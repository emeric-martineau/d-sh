///
/// Module to download file.
///
/// Release under MIT License.
///
use std::process::Command;

#[cfg(test)]
pub mod tests;

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
