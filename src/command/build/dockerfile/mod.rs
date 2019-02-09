///
/// Module with dockerfile parameter helper.
///
/// Release under MIT License.
///
use std::path::PathBuf;

///
/// Dockerfile structure.
///
pub struct DockerfileParameter {
    /// Name of dockerfile.
    pub docker_filename: String,
    /// Context of docker file.
    pub docker_context_path: String,
}

impl DockerfileParameter {
    ///
    /// Return dockerfile parameter
    ///
    pub fn new(tmp_dir: &PathBuf) -> DockerfileParameter {
        let mut docker_filename = tmp_dir.to_owned();
        docker_filename.push("Dockerfile");

        let docker_filename = docker_filename.to_str().unwrap().to_string();
        let docker_context_path = tmp_dir.to_str().unwrap().to_string();

        DockerfileParameter {
            docker_filename: docker_filename,
            docker_context_path: docker_context_path,
        }
    }
}
