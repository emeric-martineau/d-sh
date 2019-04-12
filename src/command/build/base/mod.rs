use command::build::dockerfile::DockerfileParameter;
use command::build::{generate_dockerfile, BuildOptions};
use command::{CommandError, CommandExitCode, CommandParameter};
use config::{get_config_application, Config};
use io::InputOutputHelper;
///
/// Module to build base image.
///
/// Release under MIT License.
///
use std::path::PathBuf;

///
/// Generate template of entrypoint.
///
fn generate_entrypoint(
    entrypoint_name: &str,
    io_helper: &InputOutputHelper,
    output_dir: &String,
) -> Result<(), CommandError> {
    // Check if file exists
    if !io_helper.file_exits(entrypoint_name) {
        return Err(CommandError {
            msg: vec![format!(
                "The file '{}' doesn't exits. Please run 'init' command first.",
                entrypoint_name
            )],
            code: CommandExitCode::TemplateNotFound,
        });
    }

    if let Err(err) = io_helper.hardlink_or_copy_file(
        entrypoint_name,
        &format!("{}/{}", &output_dir, entrypoint_name),
    ) {
        return Err(CommandError {
            msg: vec![
                format!("Unable copy '{}' to '{}'!", entrypoint_name, output_dir),
                format!("{}", err),
            ],
            code: CommandExitCode::CannotCopyFile,
        });
    }

    Ok(())
}

///
/// Get list of dependencies.
///
fn get_dependencies(
    io_helper: &InputOutputHelper,
    config: &Config,
) -> Result<String, CommandError> {
    let mut list_applications_file;

    // 1 - We have got configuration
    match io_helper.dir_list_file(&config.applications_dir, "*.yml") {
        Ok(r) => list_applications_file = r,
        Err(err) => {
            return Err(CommandError {
                msg: vec![
                    format!(
                        "Cannot read list of applications in '{}'!",
                        &config.applications_dir
                    ),
                    format!("{}", err),
                ],
                code: CommandExitCode::CannotReadApplicationsFolder,
            });
        }
    }

    list_applications_file.sort();
    let mut dependencies: Vec<String> = Vec::new();

    // 2 - We have list of application
    for filename in list_applications_file {
        match get_config_application(io_helper, &filename) {
            Ok(config_application) => {
                if let Some(d) = config_application.dependencies {
                    dependencies.extend(d.iter().cloned());
                }
            }
            Err(_) => {
                // Non blocking error
                io_helper.eprintln(&format!("Cannot read list of dependencies of '{}' application, please check right or file format!", &filename))
            }
        };
    }

    Ok(dependencies.join(" "))
}

///
/// Build base image.
///
pub fn build_base(
    cmd_param: &CommandParameter,
    tmp_dir: &PathBuf,
    options: &BuildOptions,
    config: &Config,
) -> Result<(), CommandError> {
    let dockerfile = DockerfileParameter::new(tmp_dir);

    let entrypoint_filename = config.entrypoint_filename.clone().unwrap();

    if let Err(err) = generate_entrypoint(
        &entrypoint_filename,
        cmd_param.io_helper,
        &dockerfile.docker_context_path) {
        return Err(err);
    }

    let mut dependencies = String::new();

    //  Get all dependencies from applications files
    if let Ok(d) = get_dependencies(cmd_param.io_helper, config) {
        dependencies = d
    }

    let data = json!({
        "dockerfile_from": config.dockerfile.from.to_owned(),
        "dockerfile_base": true,
        "dependencies": dependencies
    });

    let docker_filename = config.dockerfile_filename.clone().unwrap();

    // Generate Dockerfile
    if let Err(err) = generate_dockerfile(
        &docker_filename,
        cmd_param.io_helper,
        &dockerfile.docker_filename,
        &data) {
        return Err(err);
    }

    // Build
    let mut build_args = Vec::new();

    if options.force {
        build_args.push(String::from("--no-cache"));
    }

    if !cmd_param.dck_helper.build_image(
        &dockerfile.docker_filename,
        &dockerfile.docker_context_path,
        &config.dockerfile.tag,
        Some(&build_args),
    ) {
        return Err(CommandError {
            msg: vec![String::from("Fail to build base image!")],
            code: CommandExitCode::DockerBuildFail,
        });
    }

    Ok(())
}
