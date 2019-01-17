///
/// Module to build base image.
///
/// Release under MIT License.
///
use std::path::PathBuf;
use command::build::BuildOptions;
use docker::ContainerHelper;
use std::error::Error;
use command::CommandExitCode;
use handlebars::TemplateRenderError;
use io::InputOutputHelper;
use config::dockerfile::{DOCKERFILE_BASE_FILENAME, ENTRYPOINT_FILENAME};
use config::{Config, create_config_filename_path, get_config_application};
use template::Template;
use command::build::dockerfile::DockerfileParameter;

///
/// Generate template of dockerfile.
///
fn generate_dockerfile(config: &Config, io_helper: &InputOutputHelper, output_filename: &str, dependencies: &str) -> Result<(), CommandExitCode> {
    let handlebars = Template::new();

    let data = json!({
        "dockerfile_from": config.dockerfile.from.to_owned(),
        "dockerfile_base": true,
        "dependencies": dependencies
    });

    match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
        Some(dockerfile_name) => {
            if ! io_helper.file_exits(&dockerfile_name) {
                io_helper.eprintln(&format!("The file '{}' doesn't exits. Please run 'init' command first.", dockerfile_name));
                return Err(CommandExitCode::TemplateNotFound);
            }

            match io_helper.file_read_at_string(&dockerfile_name) {
                Ok(mut source_template) => {
                    match handlebars.render_template(&source_template, &data) {
                        Ok(content) => {
                            match io_helper.file_write(&output_filename, &content) {
                                Ok(_) => Ok(()),
                                Err(_) => {
                                    io_helper.eprintln("Unable to generate Dockerfile for build. Please check right!");
                                    Err(CommandExitCode::CannotGenerateDockerfile)
                                }
                            }
                        },
                        Err(err) => {
                            match err {
                                TemplateRenderError::TemplateError(err) => io_helper.eprintln(err.description()),
                                TemplateRenderError::RenderError(err) => io_helper.eprintln(err.description()),
                                TemplateRenderError::IOError(_, msg) => io_helper.eprintln(&msg)
                            }

                            io_helper.eprintln("Something is wrong in Dockerfile template!");
                            Err(CommandExitCode::DockerfileTemplateInvalid)
                        }
                    }
                },
                Err(_) => {
                    io_helper.eprintln("Unable to read Dockerfile template. Please check right!");
                    Err(CommandExitCode::CannotGenerateDockerfile)
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

            // Generate Dockerfile
            match generate_dockerfile(&config, io_helper, &dockerfile.docker_filename, &dependencies) {
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
