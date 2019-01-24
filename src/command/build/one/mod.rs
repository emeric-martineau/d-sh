///
/// Module to build one image.
///
/// Release under MIT License.
///
use std::path::PathBuf;
use command::CommandError;
use command::CommandExitCode;
use command::build::BuildOptions;
use command::build::generate_dockerfile;
use command::build::dockerfile::DockerfileParameter;
use io::{InputOutputHelper, convert_path};
use docker::ContainerHelper;
use config::{Config, get_filename, get_config_application, ConfigApplication};
use download::DownloadHelper;
use std::error::Error;

///
/// Download file with curl.
///
fn download_file(app: &str, config_application: &ConfigApplication, config: &Config,
    io_helper: &InputOutputHelper, dl_helper: &DownloadHelper) -> Result<(), CommandError> {
    // Check if file already downloaded
    let app_dwn_filename = get_filename(&config.download_dir,
        &config_application.download_filename, None);
    let app_dwn_filename = convert_path(&app_dwn_filename);

    if io_helper.file_exits(&app_dwn_filename) {
        // Download file with curl
        if ! dl_helper.download_if_update(&config_application.url, &app_dwn_filename) {
            return Err(CommandError {
                msg: vec![format!("Unable to download application '{}'!", app)],
                code: CommandExitCode::UnableDownloadApplication
            });
        }
    } else {
        // Download file with curl
        if ! dl_helper.download(&config_application.url, &app_dwn_filename) {
            return Err(CommandError {
                msg: vec![format!("Unable to download application '{}'!", app)],
                code: CommandExitCode::UnableDownloadApplication
            });
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
    options: &BuildOptions, config: &Config, app: &str, dl_helper: &DownloadHelper) -> Result<(), CommandError> {

    let app_filename = convert_path(&get_filename(&config.applications_dir, app, Some(&".yml")));

    let dockerfile = DockerfileParameter::new(tmp_dir);

    let config_application;

    match get_config_application(io_helper, &app_filename) {
        Ok(r) => config_application = r,
        Err(err) => return Err(CommandError {
                msg: vec![
                    format!("Unable to find application '{}' or something is wrong in file! {}",
                        app, err.description())
                    ],
                code: CommandExitCode::ApplicationFileNotFound
            })
    }

    if let Err(err) = download_file(app, &config_application, config, io_helper, dl_helper) {
        return Err(err);
    }

    // Now build
    let data = json!({
        "dockerfile_from": config.dockerfile.tag.to_owned(),
        "dockerfile_base": false,
        "application_filename": config_application.download_filename.to_owned()
    });

    if let Err(err) = generate_dockerfile(config, io_helper, &dockerfile.docker_filename,
        &config_application.download_filename, &data) {
        return Err(err);
    }

    // Copy file to temporary folder
    let app_dwn_filename = convert_path(&get_filename(&config.download_dir,
        &config_application.download_filename, None));

    if let Err(err) = io_helper.hardlink_or_copy_file(&app_dwn_filename,
        &format!("{}/{}", &dockerfile.docker_context_path, &config_application.download_filename)) {
        return Err(CommandError {
            msg: vec![
                format!("Unable copy '{}' to '{}'!", &app_dwn_filename,
                    &dockerfile.docker_context_path),
                format!("{}", err)
                ],
            code: CommandExitCode::CannotCopyFile
        });
    }

    // Build
    let mut build_args = Vec::new();

    if options.force {
        build_args.push(String::from("--no-cache"));
    }

    if ! dck_helper.build_image(&dockerfile.docker_filename,
        &dockerfile.docker_context_path, &config_application.image_name,
        Some(&build_args)) {
        return Err(CommandError {
            // TODO test error message
            msg: vec![format!("Cannot build application {}!", app)],
            code: CommandExitCode::DockerBuildFail
        });
    }

    return Ok(());
}
