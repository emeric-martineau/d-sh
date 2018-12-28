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
    /// Run a image.
    /// `image_name` is docker image
    /// `run_options` is option of docker like volume, port...
    /// `cmd` is optional command to run in container
    /// `cmd_options` is optional option of cmd
    fn run_container(&self, image_name: &str, run_options: Option<&Vec<String>>, cmd: Option<&str>,
        cmd_options: Option<&Vec<String>>) -> bool;
    /// Build a docker image
    /// `docker_filename` is path of docker_filename
    /// `docker_context_path` is context of build
    /// `docker_tag` is docker tag
    /// `build_options` is docker build args (--build-args)
    fn build_image(&self, docker_filename: &str, docker_context_path: &str, docker_tag: &str,
        build_options: Option<&Vec<String>>) -> bool;
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

    fn run_container(&self, image_name: &str, run_options: Option<&Vec<String>>, cmd: Option<&str>,
        cmd_options: Option<&Vec<String>>) -> bool {
        // docker run
        let mut args = vec![String::from("container"), String::from("run")];

        // -v /tmp/.X11-unix/:/tmp/.X11-unix/
        // -v /dev/shm:/dev/shm
        // -v ${HOME}:/home/${USER}
        // -e DISPLAY
        // -e USERNAME_TO_RUN=${USER}
        // -e USERNAME_TO_RUN_GID=${GID}
        // -e USERNAME_TO_RUN_UID=${UID}
        if run_options.is_some() {
            for opt in run_options.unwrap() {
                args.push(opt.to_string());
            }
        }

        // ${APPLICATION_IMAGE_DOCKER}
        args.push(String::from(image_name));

        // ${APPLICATION_COMMAND_LINE}
        if cmd.is_some() {
            args.push(String::from(cmd.unwrap()));
        }

        // $@
        if cmd_options.is_some() {
            for opt in cmd_options.unwrap() {
                args.push(opt.to_string());
            }
        }

        match Command::new("docker")
            .args(&args)
            .status() {
           Ok(status) => status.success(),
           Err(_) => false
        }
    }

    fn build_image(&self, docker_filename: &str, docker_context_path: &str, docker_tag: &str,
        build_options: Option<&Vec<String>>) -> bool {
            // docker build
            let mut args = vec![String::from("image"), String::from("build")];

            if build_options.is_some() {
                for opt in build_options.unwrap() {
                    args.push(opt.to_string());
                }
            }

            args.push(String::from("-t"));
            args.push(String::from(docker_tag));

            args.push(String::from("-f"));
            args.push(String::from(docker_filename));

            // PATH
            args.push(String::from(docker_context_path));

            match Command::new("docker")
                .args(&args)
                .status() {
               Ok(status) => status.success(),
               Err(_) => false
            }
    }
}

#[cfg(test)]
pub mod tests {
    use super::ContainerHelper;
    use std::clone::Clone;
    use std::cell::RefCell;
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
        pub builds: RefCell<Vec<TestBuildImage>>
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
                builds: RefCell::new(Vec::new())
            }
        }
    }
}
