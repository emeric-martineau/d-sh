///
/// Module to check build container.
///
/// Release under MIT License.
///
use std::path::Path;
use command::{Command, CommandExitCode, CommandError, CommandParameter};
use config::{get_config_application, Config};

#[cfg(test)]
mod tests;

///
/// Structure of application.
///
pub struct CheckApplication {
    /// Name of application.
    pub name: String,
    /// Docker image name.
    pub image_name: String,
    /// If image is already build.
    pub is_build: bool,
    /// If cannot read application config file.
    pub is_error: bool,
    /// Config filename.
    pub config_filename: String
}

///
/// Return list of applications and their status.
///
pub fn get_check_application(cmd_param: &CommandParameter,
    config: &Config) -> Result<Vec<CheckApplication>, CommandError> {
    let list_applications_file;

    // 1 - We have got configuration
    match cmd_param.io_helper.dir_list_file(&config.applications_dir, "*.yml") {
        Ok(r) => list_applications_file = r,
        Err(err) => return Err(CommandError {
            msg: vec![format!("{}", err)],
            code: CommandExitCode::CannotReadApplicationsFolder
        })
    };

    let mut result = Vec::new();

    // 2 - We have list of application
    for filename in list_applications_file  {
        let application_name = Path::new(&filename)
            .file_stem()
            .unwrap()   // get OsStr
            .to_str()
            .unwrap();

        let mut app = CheckApplication {
            name: String::from(application_name),
            image_name: String::new(),
            is_build: false,
            is_error: true,
            config_filename: String::new()
        };

        if let Ok(config_application) = get_config_application(cmd_param.io_helper, &filename) {
            let images = cmd_param.dck_helper.list_image(&config_application.image_name);

            app.is_build = images.len() > 0;
            app.image_name = config_application.image_name.clone();
            app.config_filename = filename.clone();
            app.is_error = false;
        }

        result.push(app);
    };

    Ok(result)
}

///
/// Function to implement check D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn check(cmd_param: CommandParameter) -> Result<(), CommandError> {

    let config = cmd_param.config.unwrap();
    let list_applications;

    // 1 - We have got configuration
    match get_check_application(&cmd_param, &config) {
        Ok(r) => list_applications = r,
        Err(err) => return Err(err)
    }

    let error_filename: Vec<String> = list_applications
        .iter()
        .filter(|a| a.is_error)
        .map(|a| a.config_filename.clone())
        .collect();

    let list_app: Vec<CheckApplication> = list_applications
        .into_iter()
        .filter(|a| !a.is_error)
        .collect();

    // 2 - We have list of application
    for app in list_app  {
        let status;

        if app.is_build {
            status = "Build done"
        } else {
            status = "Build need";
        }

        cmd_param.io_helper.println(&format!(
            "{:<with_first$}{:<with_first$}{:<width_second$}",
            app.name,
            app.image_name,
            status,
            with_first = 34,
            width_second = 13));
    };

    if error_filename.len() == 0 {
        Ok(())
    } else {
        let mut msg_error = Vec::new();

        for filename in error_filename {
             msg_error.push(format!("The file {} have bad format!", &filename));
        }

        Err(CommandError {
            msg: msg_error,
            code: CommandExitCode::BadApplicationFormat
        })
    }
}

///
/// The `check` command.
///
pub const CHECK: Command = Command {
    /// This command call by `check`.
    name: "check",
    /// description.
    description: "List missing container image",
    /// Short name.
    short_name: "chk",
    /// `check` command have no parameter.
    min_args: 0,
    max_args: 0,
    /// `check` command have no help.
    usage: "",
    need_config_file: true,
    exec_cmd: check
};
