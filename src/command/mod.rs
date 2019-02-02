///
/// Module with command module.
///
/// Release under MIT License.
///
pub mod build;
pub mod check;
pub mod delete;
pub mod init;
pub mod list;
pub mod run;
#[cfg(test)]
pub mod tests;

use io::InputOutputHelper;
use config::{get_config_filename, Config, get_config};
use docker::ContainerHelper;
use download::DownloadHelper;

///
/// Exit code of command.
///
#[derive(Debug, PartialEq)]
pub enum CommandExitCode {
    Todo = -1,
    Ok = 0,
    ConfigFileNotFound = 1,
    CannotAccessToFolderOfConfigFile = 2,
    BadArgument = 3,
    BadApplicationFormat = 4,
    CannotReadApplicationsFolder = 5,
    UnknowOption = 6,
    CannotGetHomeFolder = 7,
    ConfigFileExits = 8,
    CannotCreateFolderForConfigFile = 9,
    CannotWriteConfigFile = 10,
    Help = 11,
    CommandNotFound = 12,
    ContainerImageNotFound = 13,
    ApplicationFileNotFound = 14,
    CannotGetCurrentUser = 15,
    ContainerRunError = 16,
    ApplicationNameMissing = 17,
    CannotGenerateDockerfile = 18,
    DockerfileTemplateInvalid = 19,
    CannotCreateFolder = 20,
    CannotDeleteTemporaryFolder = 21,
    CannotCopyFile = 22,
    ConfigFileFormatWrong = 23,
    TemplateNotFound = 24,
    UnableDownloadApplication = 25,
    DockerBuildFail = 26
}

///
/// Return of command with error number and error message to display.
///
pub struct CommandError {
    msg: Vec<String>,
    code: CommandExitCode
}

///
/// Command structure
///
pub struct Command {
    /// Name of command, like delete.
    pub name: &'static str,
    /// Help description in general help.
    pub description: &'static str,
    /// Short name like rm.
    pub short_name: &'static str,
    /// Minimum arguments of command.
    pub min_args: usize,
    /// Maximum arguments of command.
    pub max_args: usize,
    /// Usage for help command.
    pub usage: &'static str,
    /// If command need config file exists.
    pub need_config_file: bool,
    /// Execute Command.
    pub exec_cmd: fn(command: &Command, args: &[String], io_helper: &InputOutputHelper,
        dck_helper: &ContainerHelper, dl_helper: &DownloadHelper,
        config: Option<&Config>) -> Result<(), CommandError>
}

impl Command {
    ///
    /// Execute code of command
    ///
    /// `args` parameter is command line arguments of D-SH.
    ///
    /// returning exit code of D-SH
    ///
    pub fn exec(&self, args: &[String], io_helper: &InputOutputHelper,
        dck_helper: &ContainerHelper, dl_helper: &DownloadHelper) -> CommandExitCode {

        // Check parameter
        if args.len() < self.min_args || args.len() > self.max_args {
            io_helper.eprintln(&format!("\"d-sh {}\" bad arguments number.", self.name));
            io_helper.eprintln(&format!("See 'd-sh {} --help'.", self.name));

            return CommandExitCode::BadArgument;
        }

        if self.need_config_file {
            let config_file;

            match get_config_filename() {
                Some(r) => config_file = r,
                None => {
                    io_helper.eprintln("Cannot access to folder where config must be.");
                    return CommandExitCode::CannotAccessToFolderOfConfigFile;
                }
            }

            if ! io_helper.file_exits(&config_file) {
                io_helper.eprintln(
                    &format!("The file '{}' doesn't exits. Please run 'init' command first.",
                    config_file));
                return CommandExitCode::ConfigFileNotFound;
            }

            let config;

            match get_config(io_helper) {
                Ok(r) => config = r,
                Err(_) => {
                    io_helper.eprintln("Cannot read config file, please check rigts and format!");
                    return CommandExitCode::ConfigFileFormatWrong;
                }
            }

            if let Err(err) = (self.exec_cmd)(self, &args, io_helper, dck_helper, dl_helper, Some(&config)) {
                for err_msg in &err.msg {
                    io_helper.eprintln(err_msg);
                }

                return err.code;
            }
        } else {
            if let Err(err) = (self.exec_cmd)(self, &args, io_helper, dck_helper, dl_helper, None) {
                for err_msg in &err.msg {
                    io_helper.eprintln(err_msg);
                }

                return err.code;
            }
        }

        CommandExitCode::Ok
    }
}
