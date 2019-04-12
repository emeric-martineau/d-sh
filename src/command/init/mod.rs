///
/// Module to init config file.
///
/// Release under MIT License.
///
use command::{Command, CommandError, CommandExitCode, CommandParameter};
use config::create_config_filename_path;
use config::dockerfile::{
    DOCKERFILE_BASE, DOCKERFILE_BASE_FILENAME, DOCKERFILE_DEFAULT_FROM, DOCKERFILE_DEFAULT_TAG,
    ENTRYPOINT, ENTRYPOINT_FILENAME,
};

use io::InputOutputHelper;
use std::collections::HashMap;
use std::path::Path;

#[cfg(test)]
mod tests;

/// Default directory of downloading applictions.
const DOWNLOAD_DIR: &str = "~/.d-sh/download";
/// Default directory to store applications.
const APPLICATIONS_DIR: &str = "~/.d-sh/applications";

fn create_dockerfile(io_helper: &InputOutputHelper) -> Result<(), CommandError> {
    let dockerfile_list: HashMap<&str, &str> = [
        (DOCKERFILE_BASE_FILENAME, DOCKERFILE_BASE),
        (ENTRYPOINT_FILENAME, ENTRYPOINT),
    ]
    .iter()
    .cloned()
    .collect();

    // Create all docker file
    for (k, v) in &dockerfile_list {
        match create_config_filename_path(&k) {
            Some(dockerfile_name) => {
                if let Err(err) = io_helper.file_write(&dockerfile_name, &v) {
                    return Err(CommandError {
                        msg: vec![format!("Unable to write file '{}'", k), format!("{}", err)],
                        code: CommandExitCode::CannotWriteConfigFile,
                    });
                }
            }
            None => {
                return Err(CommandError {
                    msg: vec![String::from("Unable to get your home dir!")],
                    code: CommandExitCode::CannotGetHomeFolder,
                });
            }
        }
    }

    Ok(())
}

///
/// Read a line from stdin.
///
fn read_line_with_default_value(
    io_helper: &InputOutputHelper,
    prompt: &str,
    default_value: &str,
) -> String {
    let mut new_prompt = String::from(prompt);

    new_prompt.push_str(&format!(" (default: {})", default_value));

    io_helper.print(&new_prompt);

    let value = io_helper.read_line();
    let mut value = value.trim();

    // Default value
    if value.is_empty() {
        value = default_value;
    }

    String::from(value)
}

///
/// Function to implement check D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn init(cmd_param: CommandParameter) -> Result<(), CommandError> {
    if cmd_param.io_helper.file_exits(&cmd_param.config_filename) {
        return Err(CommandError {
            msg: vec![format!(
                "The file '{}' exits. Please remove it (or rename) and rerun this command.",
                cmd_param.config_filename
            )],
            code: CommandExitCode::ConfigFileExits,
        });
    }

    let download_dir = read_line_with_default_value(
        cmd_param.io_helper,
        "Enter the path of download directory",
        DOWNLOAD_DIR,
    );
    let applications_dir = read_line_with_default_value(
        cmd_param.io_helper,
        "Enter the path of applications directory",
        APPLICATIONS_DIR,
    );
    let docker_image_from = read_line_with_default_value(
        cmd_param.io_helper,
        "Enter the default docker image from",
        DOCKERFILE_DEFAULT_FROM,
    );
    let docker_image_tag = read_line_with_default_value(
        cmd_param.io_helper,
        "Enter the base image docker tag",
        DOCKERFILE_DEFAULT_TAG,
    );

    let data = format!("---\ndownload_dir: \"{}\"\napplications_dir: \"{}\"\ndockerfile:\n  from: \"{}\"\n  tag: \"{}\"\n",
        download_dir, applications_dir, docker_image_from, docker_image_tag);

    // Create folder
    let path = Path::new(&cmd_param.config_filename);

    if let Some(parent) = path.parent() {
        // No parent ? Not a error.
        if let Err(err) = cmd_param.io_helper.create_dir_all(parent.to_str().unwrap()) {
            return Err(CommandError {
                msg: vec![
                    format!("Cannot create folder '{}'!", parent.display()),
                    format!("{}", err),
                ],
                code: CommandExitCode::CannotCreateFolderForConfigFile,
            });
        }
    }

    if let Err(err) = cmd_param.io_helper.file_write(&cmd_param.config_filename, &data) {
        return Err(CommandError {
            msg: vec![
                format!("Unable to write file '{}'", cmd_param.config_filename),
                format!("{}", err),
            ],
            code: CommandExitCode::CannotWriteConfigFile,
        });
    }

    create_dockerfile(cmd_param.io_helper)
}

///
/// The `check` command.
///
pub const INIT: Command = Command {
    /// This command call by `check`.
    name: "init",
    /// description.
    description: "Initialize config file if not exists",
    /// Short name.
    short_name: "i",
    /// `check` command have no parameter.
    min_args: 0,
    max_args: 0,
    /// `check` command have no command.help.
    usage: "",
    need_config_file: false,
    exec_cmd: init,
};
