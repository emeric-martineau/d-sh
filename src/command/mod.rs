///
/// Module with command module.
///
/// Release under MIT License.
///
pub mod build;
pub mod check;
pub mod delete;
pub mod help;
pub mod init;
pub mod list;
pub mod run;
#[cfg(test)]
pub mod tests;

use config::{get_config, get_config_filename, create_config_filename_path, Config};
use docker::ContainerHelper;
use config::dockerfile::{ENTRYPOINT_FILENAME, DOCKERFILE_BASE_FILENAME};
use download::DownloadHelper;
use io::{InputOutputHelper, convert_path};
use log::LoggerHelper;

use self::build::BUILD;
use self::check::CHECK;
use self::delete::DELETE;
use self::init::INIT;
use self::list::LIST;
use self::run::RUN;
use self::help::HELP;
use self::help::VERSION;

pub const ALL_COMMANDS: &'static [Command] = &[BUILD, CHECK, DELETE, HELP, INIT, LIST, RUN, VERSION];

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
    DockerBuildFail = 26,
}

///
/// Return of command with error number and error message to display.
///
pub struct CommandError {
    msg: Vec<String>,
    code: CommandExitCode,
}

///
/// Struct with input parameters.
///
pub struct CommandParameter<'a> {
    /// Current command struct.
    pub command: &'a Command,
    /// Args of command.
    pub args: &'a [String],
    /// IO Helper.
    pub io_helper: &'a InputOutputHelper,
    /// Docker Helper.
    pub dck_helper: &'a ContainerHelper,
    /// Download Helper.
    pub dl_helper: &'a DownloadHelper,
    /// Logger helper
    pub log_helper: &'a LoggerHelper,
    /// Config of D-SH.
    pub config: Option<&'a Config>,
    /// Config filename.
    pub config_filename: String
}

///
/// Command structure
///
pub struct Command {
    /// Name of command, like delete.
    pub name: &'static str,
    /// Help description in general command.help.
    pub description: &'static str,
    /// Short name like rm.
    pub short_name: &'static str,
    /// Minimum arguments of command.
    pub min_args: usize,
    /// Maximum arguments of command.
    pub max_args: usize,
    /// Usage for command.help command.
    pub usage: &'static str,
    /// If command need config file exists.
    pub need_config_file: bool,
    /// Execute Command.
    pub exec_cmd: fn(cmd_param: CommandParameter) -> Result<(), CommandError>,
}

impl Command {
    fn get_config(config_filename : Option<String>,
                  io_helper: &InputOutputHelper,
                  log_helper: &LoggerHelper) -> Result<Config, CommandExitCode> {
        let config_file;

        match config_filename {
            Some(ref c) => config_file = convert_path(c),
            None => match get_config_filename() {
                Some(r) => config_file = r,
                None => {
                    io_helper.eprintln("Cannot access to folder where config must be.");
                    return Err(CommandExitCode::CannotAccessToFolderOfConfigFile);
                }
            }
        }

        if !io_helper.file_exits(&config_file) {
            io_helper.eprintln(&format!(
                "The file '{}' doesn't exits. Please run 'init' command first.",
                config_file
            ));
            return Err(CommandExitCode::ConfigFileNotFound);
        }

        let config;

        match get_config(config_file, io_helper) {
            Ok(r) => config = r,
            Err(e) => {
                log_helper.err(&format!("{}", e));
                println!("{}", e);
                io_helper.eprintln("Cannot read config file, please check rigts and format!");
                return Err(CommandExitCode::ConfigFileFormatWrong);
            }
        }

        let dockerfile_filename;
        let entrypoint_filename;

        match config.dockerfile_filename {
            Some(ref d) => dockerfile_filename = convert_path(d),
            None => {
                match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
                    Some(r) => dockerfile_filename = r,
                    None => {
                        io_helper.eprintln("Unable to get your home dir!");
                        return Err(CommandExitCode::CannotGetHomeFolder);
                    }
                }
            }
        }

        match config.entrypoint_filename {
            Some(ref d) => entrypoint_filename = convert_path(d),
            None => {
                match create_config_filename_path(&ENTRYPOINT_FILENAME) {
                    Some(r) => entrypoint_filename = r,
                    None => {
                        io_helper.eprintln("Unable to get your home dir!");
                        return Err(CommandExitCode::CannotGetHomeFolder);
                    }
                }
            }
        }

        Ok(Config {
            download_dir: config.download_dir.clone(),
            applications_dir: config.applications_dir.clone(),
            dockerfile: config.dockerfile.clone(),
            dockerfile_filename: Some(dockerfile_filename),
            entrypoint_filename: Some(entrypoint_filename),
            tmp_dir: config.tmp_dir.clone()
        })
    }

    ///
    /// Execute code of command
    ///
    /// `args` parameter is command line arguments of D-SH.
    ///
    /// returning exit code of D-SH
    ///
    pub fn exec(
        &self,
        args: &[String],
        config_filename : Option<String>,
        io_helper: &InputOutputHelper,
        dck_helper: &ContainerHelper,
        dl_helper: &DownloadHelper,
        log_helper: &LoggerHelper
    ) -> CommandExitCode {
        // Check parameter
        if args.len() < self.min_args || args.len() > self.max_args {
            io_helper.eprintln(&format!("\"d-sh {}\" bad arguments number.", self.name));
            io_helper.eprintln(&format!("See 'd-sh {} --command.help'.", self.name));

            return CommandExitCode::BadArgument;
        }

        if self.need_config_file {
            let config ;

            match Command::get_config(config_filename, io_helper, log_helper) {
                Ok(c) => config = c,
                Err(e) => return e
            };

            let cmd_param = CommandParameter {
                command: self,
                args,
                io_helper,
                dck_helper,
                dl_helper,
                log_helper,
                config: Some(&config),
                config_filename: get_config_filename().unwrap()
            };

            if let Err(err) = (self.exec_cmd)(cmd_param) {
                for err_msg in &err.msg {
                    io_helper.eprintln(err_msg);
                }

                return err.code;
            }
        } else {
            let cmd_param = CommandParameter {
                command: self,
                args,
                io_helper,
                dck_helper,
                dl_helper,
                log_helper,
                config: None,
                config_filename: get_config_filename().unwrap()
            };

            if let Err(err) = (self.exec_cmd)(cmd_param) {
                for err_msg in &err.msg {
                    io_helper.eprintln(err_msg);
                }

                return err.code;
            }
        }

        CommandExitCode::Ok
    }
}
