///
/// Module to build base image.
///
/// Release under MIT License.
///
use std::path::PathBuf;
use command::CommandError;
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
fn generate_entrypoint(io_helper: &InputOutputHelper, output_dir: &String) -> Result<(), CommandError> {
    let entrypoint_name;

    match create_config_filename_path(&ENTRYPOINT_FILENAME) {
        Some(r) => entrypoint_name = r,
        None =>  return Err(CommandError {
            msg: vec![String::from("Unable to get your home dir!")],
            code: CommandExitCode::CannotGetHomeFolder
        })
    }

    // Check if file exists
    if ! io_helper.file_exits(&entrypoint_name) {
        return Err(CommandError {
            msg: vec![format!("The file '{}' doesn't exits. Please run 'init' command first.",
                entrypoint_name)],
            code: CommandExitCode::TemplateNotFound
        });
    }

    if let Err(err) = io_helper.hardlink_or_copy_file(&entrypoint_name, &format!("{}/{}", &output_dir,
        &ENTRYPOINT_FILENAME)) {
        return Err(CommandError {
            msg: vec![
                format!("Unable copy '{}' to '{}'!", entrypoint_name, output_dir),
                format!("{}", err)
                ],
            code: CommandExitCode::CannotCopyFile
        });
    }

    Ok(())
}


///
/// Get list of dependencies.
///
fn get_dependencies(io_helper: &InputOutputHelper, config: &Config) -> Result<String, CommandError> {
    let mut list_applications_file;

    // 1 - We have got configuration
    match io_helper.dir_list_file(&config.applications_dir, "*.yml") {
        Ok(r) => list_applications_file = r,
        Err(err) => return Err(CommandError {
            msg: vec![
                format!("Cannot read list of applications in '{}'!", &config.applications_dir),
                format!("{}", err)
                ],
            code: CommandExitCode::CannotReadApplicationsFolder
        })
    }

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
}

///
/// Build base image.
///
pub fn build_base(io_helper: &InputOutputHelper, dck_helper: &ContainerHelper, tmp_dir: &PathBuf,
    options: &BuildOptions, config: &Config) -> Result<(), CommandError> {

    let dockerfile = DockerfileParameter::new(tmp_dir);

    if let Err(err) = generate_entrypoint(io_helper, &dockerfile.docker_context_path) {
        return Err(err);
    }

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
    if let Err(err) = generate_dockerfile(&config, io_helper, &dockerfile.docker_filename,
        &dependencies, &data) {
        return Err(err);
    }

    // Build
    let mut build_args = Vec::new();

    if options.force {
        build_args.push(String::from("--no-cache"));
    }

    if ! dck_helper.build_image(&dockerfile.docker_filename, &dockerfile.docker_context_path,
        &config.dockerfile.tag, Some(&build_args)) {
        return Err(CommandError {
            msg: vec![String::from("Fail to build base image!")],
            code: CommandExitCode::DockerBuildFail
        });
    }

    Ok(())
}
