///
/// Module to manage docker command.
///
/// Release under MIT License.
///
use std::process::Command;

/// Trait to write one screen.
pub trait ContainerHelper {
    /// List image.
    fn list_image(&mut self, image_name: &str) -> Vec<String>;
}

/// Default print on tty.
pub struct DefaultContainerHelper;

impl ContainerHelper for DefaultContainerHelper {
    fn list_image(&mut self, image_name: &str)  -> Vec<String> {
        // TODO check status
        let output = Command::new("docker")
            .args(&["image", "list", "--format", "{{.ID}}", image_name])
            .output()
            .expect("failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: Vec<&str> = stdout.split(r"\n").collect();
        // Convert to string
        result.iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
pub mod tests {
    use super::ContainerHelper;

    /// Use this fonction for test.
    pub struct TestContainerHelper {
        pub images: Vec<String>
    }

    impl ContainerHelper for TestContainerHelper {
        fn list_image(&mut self, image_name: &str) -> Vec<String> {
            self.images
                .iter()
                .filter(|i| *i == image_name)
                .map(|i| i.to_string())
                .collect()
        }
    }

    impl TestContainerHelper {
        pub fn new() -> TestContainerHelper {
            TestContainerHelper {
                images: Vec::new()
            }
        }
    }
}
