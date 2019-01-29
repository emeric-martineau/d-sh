///
/// Module to list all application avaible.
///
/// Release under MIT License.
///
use std::path::Path;
use command::{Command, CommandExitCode, CommandError};
use io::InputOutputHelper;
use docker::ContainerHelper;
use config::Config;
use download::DownloadHelper;

#[cfg(test)]
mod tests;

///
/// Function to get all applications list.
///
/// returning list of application or CommandError.
///
pub fn get_all(io_helper: &InputOutputHelper, config: &Config) -> Result<Vec<String>, CommandError> {
    let mut list_applications_file;

    match io_helper.dir_list_file(&config.applications_dir, "*.yml") {
        Ok(r) => list_applications_file = r,
        Err(err) => return Err(CommandError {
            msg: vec![format!("{}", err)],
            code: CommandExitCode::DockerBuildFail
        })
    };

    list_applications_file.sort();

    let mut app_list = Vec::new();

    // 2 - We have list of application
    for filename in list_applications_file  {
        let application_name = Path::new(&filename)
            .file_stem()
            .unwrap()   // get OsStr
            .to_str()
            .unwrap();

        app_list.push(String::from(application_name));
    };

    Ok(app_list)
}

///
/// Function to implement list D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn list(_command: &Command, _args: &[String], io_helper: &InputOutputHelper,
    _dck_helper: &ContainerHelper, _dl_helper: &DownloadHelper,
    config: Option<&Config>) -> CommandExitCode {

    let config = config.unwrap();

    match get_all(io_helper, config) {
        Ok(list_applications_file) => {
            // 2 - We have list of application
            for app in list_applications_file {
                io_helper.println(&app);
            };

            CommandExitCode::Ok
        },
        Err(_) => CommandExitCode::CannotReadApplicationsFolder
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
    exec_cmd: list
};
