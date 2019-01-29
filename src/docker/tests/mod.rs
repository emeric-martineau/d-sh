///
/// Module to provide structure for tests.
///
/// Release under MIT License.
///
use super::ContainerHelper;
use std::clone::Clone;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

/// When build image
pub struct TestBuildImage {
        pub build_options: Vec<String>,
        pub tag: String,
        pub dockerfile_name: String,
        pub base_dir: String
}

/// When run a container
pub struct TestRunContainer {
    pub image_name: String,
    pub run_options: Vec<String>,
    pub cmd: String,
    pub cmd_options: Vec<String>
}

impl Clone for TestRunContainer {
    fn clone(&self) -> TestRunContainer {
        TestRunContainer {
            image_name: self.image_name.clone(),
            run_options: self.run_options.clone(),
            cmd: self.cmd.clone(),
            cmd_options: self.cmd_options.clone()
        }
    }
}

impl Display for TestRunContainer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let  r_opts = &self.run_options.iter()
            .map(|i| format!("\"{}\"", i))
            .collect::<Vec<String>>()
            .join(", ");
        let c_opts = &self.cmd_options.iter()
            .map(|i| format!("\"{}\"", i))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "(image_name={}, run_options=[{}], cmd={}, cmd_options=[{}])",
            self.image_name, r_opts, self.cmd, c_opts)
    }
}

/// Use this fonction for test.
pub struct TestContainerHelper {
    pub images: RefCell<Vec<String>>,
    pub containers: RefCell<Vec<TestRunContainer>>,
    pub builds: RefCell<Vec<TestBuildImage>>,
    pub builds_error: RefCell<HashMap<String, bool>>
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

    fn run_container(&self, image_name: &str, run_options: Option<&Vec<String>>, cmd: Option<&str>,
        cmd_options: Option<&Vec<String>>) -> bool {

        let nb_image = self.images.borrow()
            .iter()
            .filter(|i| *i == image_name)
            .count();

        if nb_image > 0 {
            let r_opts = match run_options {
                Some(opts) => opts.clone(),
                None => Vec::new()
            };

            let c_opts = match cmd_options {
                Some(opts) => opts.clone(),
                None => Vec::new()
            };

            let c = match cmd {
                Some(opts) => String::from(opts),
                None => String::new()
            };

            let new_running_container = TestRunContainer {
                image_name: String::from(image_name),
                run_options: r_opts,
                cmd: c,
                cmd_options: c_opts
            };

            self.containers.borrow_mut().push(new_running_container);

            true
        } else {
            false
        }
    }

    fn build_image(&self, docker_filename: &str, docker_context_path: &str, docker_tag: &str,
        build_options: Option<&Vec<String>>) -> bool {

        if self.builds_error.borrow().contains_key(docker_tag) {
            return false;
        }

        self.images.borrow_mut().push(String::from(docker_tag));

        let b_opts = match build_options {
            Some(opts) => opts.clone(),
            None => Vec::new()
        };

        let build = TestBuildImage {
            build_options:b_opts,
            tag: String::from(docker_tag),
            dockerfile_name: String::from(docker_filename),
            base_dir: String::from(docker_context_path)
        };

        self.builds.borrow_mut().push(build);

        true
    }
}

impl TestContainerHelper {
    pub fn new() -> TestContainerHelper {
        TestContainerHelper {
            images: RefCell::new(Vec::new()),
            containers: RefCell::new(Vec::new()),
            builds: RefCell::new(Vec::new()),
            builds_error: RefCell::new(HashMap::new()),
        }
    }
}
