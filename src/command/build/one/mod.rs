///
/// Module to build one image.
///
/// Release under MIT License.
///
use std::path::PathBuf;
use command::CommandExitCode;
use command::build::BuildOptions;
use command::build::generate_dockerfile;
use command::build::dockerfile::DockerfileParameter;
use io::{InputOutputHelper, convert_path};
use docker::ContainerHelper;
use config::{Config, get_filename, get_config_application, ConfigApplication};
use download::DownloadHelper;

///
/// Download file with curl.
///
fn download_file(app: &str, config_application: &ConfigApplication, config: &Config,
    io_helper: &InputOutputHelper, dl_helper: &DownloadHelper) -> Result<(), CommandExitCode> {
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
            return Err(CommandExitCode::UnableDownloadApplication);
        }
    }

    Ok(())
}

///
/// Build one application.
///
/// Return false if application build fail.
///
pub fn build_one_application(io_helper: &InputOutputHelper, dck_helper: &ContainerHelper, tmp_dir: &PathBuf,
    options: &BuildOptions, config: &Config, app: &str, dl_helper: &DownloadHelper) -> Result<(), CommandExitCode> {

    let app_filename = convert_path(&get_filename(&config.applications_dir, app, Some(&".yml")));

    let dockerfile = DockerfileParameter::new(tmp_dir);

    match get_config_application(io_helper, &app_filename) {
        Ok(config_application) => {
            match download_file(app, &config_application, config, io_helper, dl_helper) {
                Ok(_) => {
                    // Now build
                    let data = json!({
                        "dockerfile_from": config.dockerfile.tag.to_owned(),
                        "dockerfile_base": false,
                        "application_filename": config_application.download_filename.to_owned()
                    });

                    match generate_dockerfile(config, io_helper, &dockerfile.docker_filename,
                        &config_application.download_filename, &data) {
                        Ok(_) => {
                            // Copy file to temporary folder
                            let app_dwn_filename = convert_path(&get_filename(&config.download_dir,
                                &config_application.download_filename, None));

                            if io_helper.hardlink_or_copy_file(&app_dwn_filename,
                                &format!("{}/{}", &dockerfile.docker_context_path, &config_application.download_filename)).is_err() {
                                io_helper.eprintln(&format!("Unable copy '{}' to '{}'!", &app_dwn_filename, &dockerfile.docker_context_path));

                                return Err(CommandExitCode::CannotCopyFile);
                            }

                            // Build
                            let mut build_args = Vec::new();

                            if options.force {
                                build_args.push(String::from("--no-cache"));
                            }

                            if ! dck_helper.build_image(&dockerfile.docker_filename,
                                &dockerfile.docker_context_path, &config_application.image_name,
                                Some(&build_args)) {
                                return Err(CommandExitCode::DockerBuildFail)    
                            }
                        },
                        Err(err) => return Err(err)
                    }
                },
                Err(err) => return Err(err)
            }

            return Ok(());
        },
        Err(_) => {
            io_helper.eprintln(&format!("Unable to find application '{}' or something is wrong in file!", app));
            Err(CommandExitCode::ApplicationFileNotFound)
        }
    }
}
