///
/// Module to build one image.
///
/// Release under MIT License.
///
use std::path::PathBuf;
use command::build::BuildOptions;
use io::{InputOutputHelper, convert_path};
use docker::ContainerHelper;
use config::{Config, get_filename, get_config_application, ConfigApplication, create_config_filename_path};
use download::DownloadHelper;
use command::build::dockerfile::DockerfileParameter;
use std::error::Error;
use command::CommandExitCode;
use handlebars::TemplateRenderError;
use config::dockerfile::DOCKERFILE_BASE_FILENAME;
use template::Template;
///
/// Generate template of dockerfile.
///
fn generate_dockerfile(config: &Config, io_helper: &InputOutputHelper, output_filename: &str,
    application_filename: &str) -> Result<(), CommandExitCode> {
    let handlebars = Template::new();

    let data = json!({
        "dockerfile_from": config.dockerfile.tag.to_owned(),
        "dockerfile_base": false,
        "application_filename": application_filename
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
/// Download file with curl.
///
fn download_file(app: &str, config_application: &ConfigApplication, config: &Config,
    io_helper: &InputOutputHelper, dl_helper: &DownloadHelper) -> bool {
    // Check if file already downloaded
    let app_dwn_filename = get_filename(&config.download_dir,
        &config_application.download_filename, None);
    let app_dwn_filename = convert_path(&app_dwn_filename);

    if io_helper.file_exits(&app_dwn_filename) {
        // TODO
    } else {
        // Download file with curl
        if ! dl_helper.download(&config_application.url, &app_dwn_filename) {
            io_helper.eprintln(&format!("Unable to download application '{}'!", app));
            return false;
        }
    }

    true
}

///
/// Build one application.
///
/// Return false if application build fail.
///
pub fn build_one_application(io_helper: &InputOutputHelper, dck_helper: &ContainerHelper, tmp_dir: &PathBuf,
    options: &BuildOptions, config: &Config, app: &str, dl_helper: &DownloadHelper) -> bool {

    let app_filename = convert_path(&get_filename(&config.applications_dir, app, Some(&".yml")));

    let dockerfile = DockerfileParameter::new(tmp_dir);

    match get_config_application(io_helper, &app_filename) {
        Ok(config_application) => {
            if download_file(app, &config_application, config, io_helper, dl_helper) {
                // Now build
                match generate_dockerfile(config, io_helper, &dockerfile.docker_filename,
                    &config_application.download_filename) {
                    Ok(_) => {
                        // Copy file to temporary folder
                        let app_dwn_filename = convert_path(&get_filename(&config.download_dir,
                            &config_application.download_filename, None));

                        if io_helper.hardlink_or_copy_file(&app_dwn_filename,
                            &format!("{}/{}", &dockerfile.docker_context_path, &config_application.download_filename)).is_err() {
                            io_helper.eprintln(&format!("Unable copy '{}' to '{}'!", &app_dwn_filename, &dockerfile.docker_context_path));
                            return false;
                        }

                        // Build
                        let mut build_args = Vec::new();

                        if options.force {
                            build_args.push(String::from("--no-cache"));
                        }

                        return dck_helper.build_image(&dockerfile.docker_filename,
                            &dockerfile.docker_context_path, &config_application.image_name,
                            Some(&build_args));
                    },
                    Err(_) => return false
                }
            }

            return false;
        },
        Err(_) => {
            // TODO return CommandExitCode::ApplicationFileNotFound
            io_helper.eprintln(&format!("Unable to find application '{}' or something is wrong in file!", app));
            return false;
        }
    }

}
