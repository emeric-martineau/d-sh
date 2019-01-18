///
/// Module to build base image.
///
/// Release under MIT License.
///
use std::path::PathBuf;
use command::CommandExitCode;
use command::build::BuildOptions;
use command::build::generate_dockerfile;
use command::build::dockerfile::DockerfileParameter;
use docker::ContainerHelper;
use io::InputOutputHelper;
use config::dockerfile::ENTRYPOINT_FILENAME;
use config::{Config, create_config_filename_path, get_config_application};

///
/// Generate template of entrypoint.
///
fn generate_entrypoint(io_helper: &InputOutputHelper, output_dir: &String) -> Result<(), CommandExitCode> {
    match create_config_filename_path(&ENTRYPOINT_FILENAME) {
        Some(entrypoint_name) => {
            // Check if file exists
            if ! io_helper.file_exits(&entrypoint_name) {
                io_helper.eprintln(&format!("The file '{}' doesn't exits. Please run 'init' command first.", entrypoint_name));
                return Err(CommandExitCode::TemplateNotFound);
            }

            match io_helper.hardlink_or_copy_file(&entrypoint_name, &format!("{}/{}", &output_dir, &ENTRYPOINT_FILENAME)) {
                Ok(_) => Ok(()),
                Err(_) => {
                    io_helper.eprintln(&format!("Unable copy '{}' to '{}'!", entrypoint_name, output_dir));
                    Err(CommandExitCode::CannotCopyFile)
                }
            }
        },
        None => {
            io_helper.eprintln("Unable to get your home dir!");
            Err(CommandExitCode::CannotGetHomeFolder)
        }
    }
}


///
/// Get list of dependencies.
///
fn get_dependencies(io_helper: &InputOutputHelper, config: &Config) -> Result<String, CommandExitCode> {
    // 1 - We have got configuration
    match io_helper.dir_list_file(&config.applications_dir, "*.yml") {
        Ok(mut list_applications_file) => {
            list_applications_file.sort();
            let mut dependencies: Vec<String> = Vec::new();

            // 2 - We have list of application
            for filename in list_applications_file  {
                match get_config_application(io_helper, &filename) {
                    Ok(config_application) => {
                        if let Some(d) = config_application.dependencies {
                            dependencies.extend(d.iter().cloned());
                        }
                    },
                    Err(_) => {
                        // Non blocking error
                        io_helper.eprintln(&format!("Cannot read list of dependencies of '{}' application, please check right or file format!", &filename))
                    }
                };
            };

            Ok(dependencies.join(" "))
        },
        Err(_) => Err(CommandExitCode::CannotReadApplicationsFolder)
    }
}

///
/// Build base image.
///
pub fn build_base(io_helper: &InputOutputHelper, dck_helper: &ContainerHelper, tmp_dir: &PathBuf,
    options: &BuildOptions, config: &Config) -> CommandExitCode {

    let dockerfile = DockerfileParameter::new(tmp_dir);

    match generate_entrypoint(io_helper, &dockerfile.docker_context_path) {
        Ok(_) => {
            let mut dependencies = String::new();

            //  Get all dependencies from applications files
            if let Ok(d) = get_dependencies(io_helper, config) {
                dependencies = d
            }

            let data = json!({
                "dockerfile_from": config.dockerfile.from.to_owned(),
                "dockerfile_base": true,
                "dependencies": dependencies
            });

            // Generate Dockerfile
            match generate_dockerfile(&config, io_helper, &dockerfile.docker_filename,
                &dependencies, &data) {
                Ok(_) => {
                    // Build
                    let mut build_args = Vec::new();

                    if options.force {
                        build_args.push(String::from("--no-cache"));
                    }

                    dck_helper.build_image(&dockerfile.docker_filename, &dockerfile.docker_context_path,
                        &config.dockerfile.tag, Some(&build_args));
                },
                Err(err) => return err
            }

            CommandExitCode::Ok
        },
        Err(err) => err
    }

}
