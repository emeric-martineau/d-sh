///
/// Module with command module.
///
/// Release under MIT License.
///
pub mod check;
pub mod delete;
pub mod init;
pub mod list;
pub mod run;
pub mod build;

use io::InputOutputHelper;
use config::{get_config_filename, Config, get_config};
use docker::ContainerHelper;
use process::RunCommandHelper;

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
    TemplateNotFound = 24
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
        dck_helper: &ContainerHelper, run_command_helper: &RunCommandHelper,
        config: Option<&Config>) -> CommandExitCode
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
        dck_helper: &ContainerHelper, run_command_helper: &RunCommandHelper) -> CommandExitCode {
        let exit_code;

        // Check parameter
        if args.len() >= self.min_args && args.len() <= self.max_args {
            if self.need_config_file {
                match get_config_filename() {
                    Some(config_file) => {
                        if io_helper.file_exits(&config_file) {
                            match get_config(io_helper) {
                                Ok(config) => exit_code = (self.exec_cmd)(self, &args, io_helper, dck_helper, run_command_helper, Some(&config)),
                                Err(_) => {
                                    io_helper.eprintln("Cannot read config file, please check rigts and format!");
                                    exit_code = CommandExitCode::ConfigFileFormatWrong;
                                }
                            }
                        } else {
                            io_helper.eprintln(&format!("The file '{}' doesn't exits. Please run 'init' command first.", config_file));
                            exit_code = CommandExitCode::ConfigFileNotFound;
                        }
                    },
                    None => {
                        io_helper.eprintln("Cannot access to folder where config must be.");
                        exit_code = CommandExitCode::CannotAccessToFolderOfConfigFile;
                    }
                };
            } else {
                exit_code = (self.exec_cmd)(self, &args, io_helper, dck_helper, run_command_helper, None);
            }

        } else {
            io_helper.eprintln(&format!("\"d-sh {}\" bad arguments number.", self.name));
            io_helper.eprintln(&format!("See 'd-sh {} --help'.", self.name));

            exit_code = CommandExitCode::BadArgument;
        }

        exit_code
    }
}

#[cfg(test)]
mod tests {
    use io::InputOutputHelper;
    use io::tests::TestInputOutputHelper;
    use config::{get_config_filename, Config};
    use super::{Command, CommandExitCode};
    use docker::ContainerHelper;
    use docker::tests::TestContainerHelper;
    use process::RunCommandHelper;
    use process::tests::TestRunCommandHelper;

    fn test_help(_command: &Command, _args: &[String], io_helper: &InputOutputHelper,
        _dck_helper: &ContainerHelper, _run_command_helper: &RunCommandHelper,
        _config: Option<&Config>) -> CommandExitCode {
        io_helper.println(&format!("Coucou !"));
        CommandExitCode::Ok
    }

    #[test]
    fn check_if_need_argument_but_not_provide() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let one_cmd = Command {
            name: "test",
            description: "It's a test",
            short_name: "tst",
            min_args: 1,
            max_args: 1,
            usage: "",
            need_config_file: false,
            exec_cmd: test_help
        };

        let commands = &[one_cmd];

        let args = [];

        let exit_code = commands[0].exec(&args, io_helper, dck_helper, run_command_helper);

        assert_eq!(exit_code, CommandExitCode::BadArgument);
    }

    #[test]
    fn check_if_too_many_argument() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let one_cmd = Command {
            name: "test",
            description: "It's a test",
            short_name: "tst",
            min_args: 1,
            max_args: 1,
            usage: "",
            need_config_file: false,
            exec_cmd: test_help
        };

        let commands = &[one_cmd];

        let args = [String::from("eeee"), String::from("eeee")];

        let exit_code = commands[0].exec(&args, io_helper, dck_helper, run_command_helper);

        assert_eq!(exit_code, CommandExitCode::BadArgument);
    }

    #[test]
    fn check_if_not_enough_many_argument() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let one_cmd = Command {
            name: "test",
            description: "It's a test",
            short_name: "tst",
            min_args: 2,
            max_args: 2,
            usage: "",
            need_config_file: false,
            exec_cmd: test_help
        };

        let commands = &[one_cmd];

        let args = [String::from("eeee")];

        let exit_code = commands[0].exec(&args, io_helper, dck_helper, run_command_helper);

        assert_eq!(exit_code, CommandExitCode::BadArgument);
    }

    #[test]
    fn check_if_need_config_file_and_not_found() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let one_cmd = Command {
            name: "test",
            description: "It's a test",
            short_name: "tst",
            min_args: 0,
            max_args: 0,
            usage: "",
            need_config_file: true,
            exec_cmd: test_help
        };

        let commands = &[one_cmd];

        let args = [];

        let exit_code = commands[0].exec(&args, io_helper, dck_helper, run_command_helper);

        assert_eq!(exit_code, CommandExitCode::ConfigFileNotFound);
    }

    #[test]
    fn check_if_need_config_file_and_found() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let one_cmd = Command {
            name: "test",
            description: "It's a test",
            short_name: "tst",
            min_args: 0,
            max_args: 0,
            usage: "",
            need_config_file: true,
            exec_cmd: test_help
        };

        let commands = &[one_cmd];

        let args = [];

        match get_config_filename() {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("---\ndownload_dir: \"dwn\"\napplications_dir: \"app\"\ndockerfile:\n  from: \"tata\"\n  tag: \"tutu\"\n"))
            },
            None => panic!("Unable to get config filename for test")
        };

        let exit_code = commands[0].exec(&args, io_helper, dck_helper, run_command_helper);

        assert_eq!(exit_code, CommandExitCode::Ok);
    }

    #[test]
    fn check_if_need_config_file_and_found_but_wrong_format() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let one_cmd = Command {
            name: "test",
            description: "It's a test",
            short_name: "tst",
            min_args: 0,
            max_args: 0,
            usage: "",
            need_config_file: true,
            exec_cmd: test_help
        };

        let commands = &[one_cmd];

        let args = [];

        match get_config_filename() {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("tutu"))
            },
            None => panic!("Unable to get config filename for test")
        };

        let exit_code = commands[0].exec(&args, io_helper, dck_helper, run_command_helper);

        assert_eq!(exit_code, CommandExitCode::ConfigFileFormatWrong);
    }
}
