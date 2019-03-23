///
/// Module to manage docker command.
///
/// Release under MIT License.
///
use std::process::Command;
use log::LoggerHelper;

#[cfg(test)]
pub mod tests;

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
    fn run_container(
        &self,
        image_name: &str,
        run_options: Option<&Vec<String>>,
        cmd: Option<&str>,
        cmd_options: Option<&Vec<String>>,
    ) -> bool;
    /// Build a docker image
    /// `docker_filename` is path of docker_filename
    /// `docker_context_path` is context of build
    /// `docker_tag` is docker tag
    /// `build_options` is docker build args (--build-args)
    fn build_image(
        &self,
        docker_filename: &str,
        docker_context_path: &str,
        docker_tag: &str,
        build_options: Option<&Vec<String>>,
    ) -> bool;
}

/// Default print on tty.
pub struct DefaultContainerHelper<'a> {
    log_helper: &'a LoggerHelper
}

impl<'a> DefaultContainerHelper<'a> {
    pub fn new(log_helper: &LoggerHelper) -> DefaultContainerHelper {
        DefaultContainerHelper {
            log_helper
        }
    }
}

impl<'a> ContainerHelper for DefaultContainerHelper<'a> {
    fn list_image(&self, image_name: &str) -> Vec<String> {
        let args = &["image", "list", "--format", "{{.ID}}", image_name];

        self.log_helper.debug_with_array("Run command 'docker{}'", args.into_iter());

        match Command::new("docker")
            .args(args)
            .output()
        {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);

                self.log_helper.debug_with_parameter("Response: {}", &stdout);

                let result: Vec<&str> = stdout.split(r"\n").collect();
                // Convert to string
                result
                    .iter()
                    .map(|s| s.to_string())
                    .filter(|s| !s.trim().is_empty()) // Remove empty line
                    .collect()
            }
            Err(e) => {
                self.log_helper.err(&e.to_string());
                Vec::new()
            },
        }
    }

    fn remove_image(&self, image_name: &str) -> bool {
        let args = &["image", "rm", image_name];

        self.log_helper.debug_with_array("Run command 'docker{}'", args.into_iter());

        match Command::new("docker")
            .args(args)
            .status()
        {
            Ok(status) => status.success(),
            Err(e) => {
                self.log_helper.err(&e.to_string());
                false
            },
        }
    }

    fn run_container(
        &self,
        image_name: &str,
        run_options: Option<&Vec<String>>,
        cmd: Option<&str>,
        cmd_options: Option<&Vec<String>>,
    ) -> bool {
        // docker run
        let mut args = vec!["container", "run"];

        // -v /tmp/.X11-unix/:/tmp/.X11-unix/
        // -v /dev/shm:/dev/shm
        // -v ${HOME}:/home/${USER}
        // -e DISPLAY
        // -e USERNAME_TO_RUN=${USER}
        // -e USERNAME_TO_RUN_GID=${GID}
        // -e USERNAME_TO_RUN_UID=${UID}
        if run_options.is_some() {
            for opt in run_options.unwrap() {
                args.push(opt);
            }
        }

        // ${APPLICATION_IMAGE_DOCKER}
        args.push(image_name);

        // ${APPLICATION_COMMAND_LINE}
        if cmd.is_some() {
            args.push(cmd.unwrap());
        }

        // $@
        if cmd_options.is_some() {
            for opt in cmd_options.unwrap() {
                args.push(opt);
            }
        }

        self.log_helper.debug_with_array("Run command 'docker{}'", args.iter());

        match Command::new("docker").args(&args).status() {
            Ok(status) => status.success(),
            Err(e) => {
                self.log_helper.warn(&e.to_string());
                false
            },
        }
    }

    fn build_image(
        &self,
        docker_filename: &str,
        docker_context_path: &str,
        docker_tag: &str,
        build_options: Option<&Vec<String>>,
    ) -> bool {
        // docker build
        let mut args = vec!["image", "build"];

        if build_options.is_some() {
            for opt in build_options.unwrap() {
                args.push(&opt);
            }
        }

        args.push("-t");
        args.push(docker_tag);

        args.push("-f");
        args.push(docker_filename);

        // PATH
        args.push(docker_context_path);

        self.log_helper.debug_with_array("Run command 'docker{}'", args.iter());

        match Command::new("docker").args(&args).status() {
            Ok(status) => status.success(),
            Err(e) => {
                self.log_helper.err(&e.to_string());
                false
            },
        }
    }
}
