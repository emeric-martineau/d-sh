///
/// Module with command module.
///
/// Release under MIT License.
///
pub mod check;
pub mod init;

use super::io::InputOutputHelper;
use super::config::get_config_filename;
use super::docker::ContainerHelper;

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
    pub exec_cmd: fn(command: &Command, args: &[String], io_helper: &mut InputOutputHelper,
        dck_helper: &mut ContainerHelper) -> i32 // TODO use enum with exit code
}

impl Command {
    ///
    /// Execute code of command
    ///
    /// `args` parameter is command line arguments of D-SH.
    ///
    /// returning exit code of D-SH
    ///
    pub fn exec(&self, args: &[String], io_helper: &mut InputOutputHelper,
        dck_helper: &mut ContainerHelper) -> i32 {
        let exit_code;

        // Check parameter
        if args.len() >= self.min_args && args.len() <= self.max_args {
            if self.need_config_file {
                match get_config_filename() {
                    Some(config_file) => {
                        if io_helper.file_exits(&config_file) {
                            exit_code = (self.exec_cmd)(self, &args, io_helper, dck_helper)
                        } else {
                            io_helper.eprintln(&format!("The file '{}' exits. Please remove it (or rename) and retry this command.", config_file));
                            exit_code = 5;
                        }
                    },
                    None => {
                        io_helper.eprintln("Enable to find d-sh config file. Please run 'd-sh init' first.");
                        exit_code = 6;
                    }
                };
            } else {
                exit_code = (self.exec_cmd)(self, &args, io_helper, dck_helper)
            }

        } else {
            io_helper.eprintln(&format!("\"d-sh {}\" bad arguments number.", self.name));
            io_helper.eprintln("See 'd-sh $1 --help'.");

            exit_code = 4
        }

        exit_code
    }
}

#[cfg(test)]
mod tests {
    use super::super::io::InputOutputHelper;
    use super::super::io::tests::TestInputOutputHelper;
    use super::Command;
    use super::get_config_filename;
    use super::super::docker::ContainerHelper;
    use super::super::docker::tests::TestContainerHelper;

    fn test_help(_command: &Command, _args: &[String], io_helper: &mut InputOutputHelper,
        _dck_helper: &mut ContainerHelper) -> i32 {
        io_helper.println(&format!("Coucou !"));
        0
    }

    #[test]
    fn check_if_need_argument_but_not_provide() {
        let io_helper = &mut TestInputOutputHelper::new();
        let dck_helper = &mut TestContainerHelper::new();

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

        let exit_code = commands[0].exec(&args, io_helper, dck_helper);

        assert_eq!(exit_code, 4);
    }

    #[test]
    fn check_if_too_many_argument() {
        let io_helper = &mut TestInputOutputHelper::new();
        let dck_helper = &mut TestContainerHelper::new();

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

        let exit_code = commands[0].exec(&args, io_helper, dck_helper);

        assert_eq!(exit_code, 4);
    }

    #[test]
    fn check_if_not_enough_many_argument() {
        let io_helper = &mut TestInputOutputHelper::new();
        let dck_helper = &mut TestContainerHelper::new();

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

        let exit_code = commands[0].exec(&args, io_helper, dck_helper);

        assert_eq!(exit_code, 4);
    }

    #[test]
    fn check_if_need_config_file_and_not_found() {
        let io_helper = &mut TestInputOutputHelper::new();
        let dck_helper = &mut TestContainerHelper::new();

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

        let exit_code = commands[0].exec(&args, io_helper, dck_helper);

        assert_eq!(exit_code, 5);
    }

    #[test]
    fn check_if_need_config_file_and_found() {
        let io_helper = &mut TestInputOutputHelper::new();
        let dck_helper = &mut TestContainerHelper::new();

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
                io_helper.files.insert(cfg_file, String::from("toto"))
            },
            None => panic!("Unable to get config filename for test")
        };

        let exit_code = commands[0].exec(&args, io_helper, dck_helper);

        assert_eq!(exit_code, 0);
    }
}
