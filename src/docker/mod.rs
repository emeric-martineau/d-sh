///
/// Module to manage docker command.
///
/// Release under MIT License.
///
use std::process::Command;

/// Trait to write one screen.
pub trait ContainerHelper {
    /// List image.
    /// Return list of image id.
    fn list_image(&self, image_name: &str) -> Vec<String>;
    /// Remove image.
    /// Return true if ok.
    fn remove_image(&self, image_name: &str) -> bool;
}

/// Default print on tty.
pub struct DefaultContainerHelper;

impl ContainerHelper for DefaultContainerHelper {
    fn list_image(&self, image_name: &str)  -> Vec<String> {
        match Command::new("docker")
            .args(&["image", "list", "--format", "{{.ID}}", image_name])
            .output() {
           Ok(output) => {
               let stdout = String::from_utf8_lossy(&output.stdout);
               let result: Vec<&str> = stdout
                   .split(r"\n")
                   .collect();
               // Convert to string
               result
                   .iter()
                   .map(|s| s.to_string())
                   .filter(|s| !s.trim().is_empty()) // Remove empty line
                   .collect()
           },
           Err(_) => Vec::new()
        }
    }

    fn remove_image(&self, image_name: &str) -> bool {
        match Command::new("docker")
            .args(&["image", "rm", image_name])
            .status() {
           Ok(status) => status.success(),
           Err(_) => false
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::ContainerHelper;
    use std::cell::RefCell;

    /// Use this fonction for test.
    pub struct TestContainerHelper {
        pub images: RefCell<Vec<String>>
    }

    impl ContainerHelper for TestContainerHelper {
        fn list_image(&self, image_name: &str) -> Vec<String> {
            self.images.borrow()
                .iter()
                .filter(|i| *i == image_name)
                .map(|i| i.to_string())
                .collect()
        }

        fn remove_image(&self, image_name: &str) -> bool {
            let nb_image = self.images.borrow()
                .iter()
                .filter(|i| *i == image_name)
                .count();

            if nb_image > 0 {
                // Remove item
                self.images.borrow_mut()
                    .retain(|i| *i != image_name);

                true
            } else {
                false
            }
        }
    }

    impl TestContainerHelper {
        pub fn new() -> TestContainerHelper {
            TestContainerHelper {
                images: RefCell::new(Vec::new())
            }
        }
    }
}
