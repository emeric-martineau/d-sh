use command::{Command, CommandError, CommandExitCode, CommandParameter};
use config::Config;
use io::InputOutputHelper;
///
/// Module to list all application avaible.
///
/// Release under MIT License.
///
use std::path::Path;

#[cfg(test)]
mod tests;

///
/// Function to get all applications list.
///
/// returning list of application or CommandError.
///
pub fn get_all(
    io_helper: &InputOutputHelper,
    config: &Config,
) -> Result<Vec<String>, CommandError> {
    let mut list_applications_file;

    match io_helper.dir_list_file(&config.applications_dir, "*.yml") {
        Ok(r) => list_applications_file = r,
        Err(err) => {
            return Err(CommandError {
                msg: vec![format!("{}", err)],
                code: CommandExitCode::DockerBuildFail,
            });
        }
    };

    list_applications_file.sort();

    let mut app_list = Vec::new();

    // 2 - We have list of application
    for filename in list_applications_file {
        let application_name = Path::new(&filename)
            .file_stem()
            .unwrap() // get OsStr
            .to_str()
            .unwrap();

        app_list.push(String::from(application_name));
    }

    Ok(app_list)
}

///
/// Function to implement list D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn list(cmd_param: CommandParameter) -> Result<(), CommandError> {
    let config = cmd_param.config.unwrap();

    match get_all(cmd_param.io_helper, &config) {
        Ok(list_applications_file) => {
            // 2 - We have list of application
            for app in list_applications_file {
                cmd_param.io_helper.println(&app);
            }

            Ok(())
        }
        Err(_) => Err(CommandError {
            msg: vec![format!(
                "Cannot read application folder {}!",
                config.applications_dir
            )],
            code: CommandExitCode::CannotReadApplicationsFolder,
        }),
    }
}

///
/// The `list` command.
///
pub const LIST: Command = Command {
    /// This command call by `check`.
    name: "list",
    /// description.
    description: "List all applications available",
    /// Short name.
    short_name: "ls",
    /// `check` command have no parameter.
    min_args: 0,
    max_args: 0,
    /// `check` command have no help.
    usage: "",
    need_config_file: true,
    exec_cmd: list,
};
