///
/// Module to delete image.
///
/// Release under MIT License.
///
use std::path::Path;
use command::{Command, CommandExitCode, CommandError};
use io::InputOutputHelper;
use docker::ContainerHelper;
use config::{Config, get_config_application, get_filename};
use download::DownloadHelper;

#[cfg(test)]
mod tests;

///
/// Function to delete one image.
///
/// `app` name of application name.
///
/// returning exit code of D-SH.
///
fn delete_one(config: &Config, app: &str, io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper)  -> Result<(), CommandError> {

    let application_filename_full_path = get_filename(&config.applications_dir, app, Some(&".yml"));

    match get_config_application(io_helper, &application_filename_full_path) {
        Ok(config_application) => {
            if dck_helper.remove_image(&config_application.image_name) {
                Ok(())
            } else {
                Err(CommandError {
                    msg: vec![String::from("Docker image not found")],
                    code: CommandExitCode::ContainerImageNotFound
                })
            }
        },
        Err(err) => Err(CommandError {
            msg: vec![format!("{}", err)],
            code: CommandExitCode::ApplicationFileNotFound
        })
    }
}

///
/// Function to delete all images.
///
/// `app` name of application name.
///
/// returning exit code of D-SH.
///
fn delete_all(config: &Config, io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper)  -> Result<(), CommandError> {
    let mut list_applications_file;

    match io_helper.dir_list_file(&config.applications_dir, "*.yml") {
        Ok(r) => list_applications_file = r,
        Err(err) => return Err(CommandError {
            msg: vec![format!("{}", err)],
            code: CommandExitCode::CannotReadApplicationsFolder
        })
    }

    list_applications_file.sort();

    // 2 - We have list of application
    for filename in list_applications_file  {
        let application_name = Path::new(&filename)
            .file_stem()
            .unwrap()   // get OsStr
            .to_str()
            .unwrap();

        if let Err(err) = delete_one(&config, &application_name, io_helper, dck_helper) {
            for err_msg in &err.msg {
                io_helper.eprintln(err_msg);
            }
        }
    };

    Ok(())
}

///
/// Function to implement delete D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn delete(command: &Command, args: &[String], io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper, _dl_helper: &DownloadHelper,
    config: Option<&Config>) -> Result<(), CommandError>  {

    let config = config.unwrap();

    match args[0].as_ref() {
        "-h" | "--help" => {
            io_helper.println(command.usage);
            Ok(())
        },
        "-a" | "--all" => {
            delete_all(&config, io_helper, dck_helper)
        },
        app => {
            delete_one(&config, app, io_helper, dck_helper)
        }
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
    exec_cmd: delete
};
