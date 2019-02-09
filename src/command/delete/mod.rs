use command::{Command, CommandError, CommandExitCode, CommandParameter};
use config::{get_config_application, get_filename, Config};
///
/// Module to delete image.
///
/// Release under MIT License.
///
use std::path::Path;

#[cfg(test)]
mod tests;

///
/// Function to delete one image.
///
/// `app` name of application name.
///
/// returning exit code of D-SH.
///
fn delete_one(
    cmd_param: &CommandParameter,
    config: &Config,
    app: &str,
) -> Result<(), CommandError> {
    let application_filename_full_path = get_filename(&config.applications_dir, app, Some(&".yml"));

    match get_config_application(cmd_param.io_helper, &application_filename_full_path) {
        Ok(config_application) => {
            if cmd_param
                .dck_helper
                .remove_image(&config_application.image_name)
            {
                Ok(())
            } else {
                Err(CommandError {
                    msg: vec![String::from("Docker image not found")],
                    code: CommandExitCode::ContainerImageNotFound,
                })
            }
        }
        Err(err) => Err(CommandError {
            msg: vec![format!("{}", err)],
            code: CommandExitCode::ApplicationFileNotFound,
        }),
    }
}

///
/// Function to delete all images.
///
/// `app` name of application name.
///
/// returning exit code of D-SH.
///
fn delete_all(cmd_param: &CommandParameter, config: &Config) -> Result<(), CommandError> {
    let mut list_applications_file;

    match cmd_param
        .io_helper
        .dir_list_file(&config.applications_dir, "*.yml")
    {
        Ok(r) => list_applications_file = r,
        Err(err) => {
            return Err(CommandError {
                msg: vec![format!("{}", err)],
                code: CommandExitCode::CannotReadApplicationsFolder,
            });
        }
    }

    list_applications_file.sort();

    // 2 - We have list of application
    for filename in list_applications_file {
        let application_name = Path::new(&filename)
            .file_stem()
            .unwrap() // get OsStr
            .to_str()
            .unwrap();

        if let Err(err) = delete_one(&cmd_param, &config, &application_name) {
            for err_msg in &err.msg {
                cmd_param.io_helper.eprintln(err_msg);
            }
        }
    }

    Ok(())
}

///
/// Function to implement delete D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn delete(cmd_param: CommandParameter) -> Result<(), CommandError> {
    let config = cmd_param.config.unwrap();

    match cmd_param.args[0].as_ref() {
        "-h" | "--help" => {
            cmd_param.io_helper.println(cmd_param.command.usage);
            Ok(())
        }
        "-a" | "--all" => delete_all(&cmd_param, &config),
        app => delete_one(&cmd_param, &config, app),
    }
}

///
/// The `delete` command.
///
pub const DELETE: Command = Command {
    /// This command call by `check`.
    name: "delete",
    /// description.
    description: "Delete image",
    /// Short name.
    short_name: "rm",
    /// `check` command have no parameter.
    min_args: 1,
    max_args: 1,
    /// `check` command have no help.
    usage: "
    Usage:	d-sh delete APPLICATION

    Delete an image for a application

    Options:
      -a, --all                Build all image of application
",
    need_config_file: true,
    exec_cmd: delete,
};
