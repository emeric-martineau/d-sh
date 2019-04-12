use super::{Command, CommandError, CommandExitCode, CommandParameter};
use docker::tests::TestContainerHelper;
use download::tests::TestDownloadHelper;
///
/// Module to tests module command.
///
/// Release under MIT License.
///
use io::tests::TestInputOutputHelper;
use log::EmptyLoggerHelper;

pub fn test_result_ok(result: Result<(), CommandError>) {
    if let Err(err) = result {
        panic!(format!(
            "Command return fail with code {:?} and error message {:?}.",
            err.code, err.msg
        ));
    }
}

pub fn test_result_err(result: Result<(), CommandError>, err_code: CommandExitCode) -> Vec<String> {
    match result {
        Ok(_) => panic!("Command should be fail but not!"),
        Err(err) => {
            assert_eq!(err.code, err_code);

            return err.msg;
        }
    }
}

fn test_help(cmd_param: CommandParameter) -> Result<(), CommandError> {
    cmd_param.io_helper.println(&format!("Coucou !"));
    Ok(())
}

#[test]
fn check_if_need_argument_but_not_provide() {
    let io_helper = &TestInputOutputHelper::new();
    let dck_helper = &TestContainerHelper::new();
    let dl_helper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    let one_cmd = Command {
        name: "test",
        description: "It's a test",
        short_name: "tst",
        min_args: 1,
        max_args: 1,
        usage: "",
        need_config_file: false,
        exec_cmd: test_help,
    };

    let commands = &[one_cmd];

    let args = [];

    let exit_code = commands[0].exec(&args, None, io_helper, dck_helper, dl_helper, log_helper);

    assert_eq!(exit_code, CommandExitCode::BadArgument);
}

#[test]
fn check_if_too_many_argument() {
    let io_helper = &TestInputOutputHelper::new();
    let dck_helper = &TestContainerHelper::new();
    let dl_helper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    let one_cmd = Command {
        name: "test",
        description: "It's a test",
        short_name: "tst",
        min_args: 1,
        max_args: 1,
        usage: "",
        need_config_file: false,
        exec_cmd: test_help,
    };

    let commands = &[one_cmd];

    let args = [String::from("eeee"), String::from("eeee")];

    let exit_code = commands[0].exec(&args, None, io_helper, dck_helper, dl_helper, log_helper);

    assert_eq!(exit_code, CommandExitCode::BadArgument);
}

#[test]
fn check_if_not_enough_many_argument() {
    let io_helper = &TestInputOutputHelper::new();
    let dck_helper = &TestContainerHelper::new();
    let dl_helper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    let one_cmd = Command {
        name: "test",
        description: "It's a test",
        short_name: "tst",
        min_args: 2,
        max_args: 2,
        usage: "",
        need_config_file: false,
        exec_cmd: test_help,
    };

    let commands = &[one_cmd];

    let args = [String::from("eeee")];

    let exit_code = commands[0].exec(&args, None, io_helper, dck_helper, dl_helper, log_helper);

    assert_eq!(exit_code, CommandExitCode::BadArgument);
}

#[test]
fn check_if_need_config_file_and_not_found() {
    let io_helper = &TestInputOutputHelper::new();
    let dck_helper = &TestContainerHelper::new();
    let dl_helper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    let one_cmd = Command {
        name: "test",
        description: "It's a test",
        short_name: "tst",
        min_args: 0,
        max_args: 0,
        usage: "",
        need_config_file: true,
        exec_cmd: test_help,
    };

    let commands = &[one_cmd];

    let args = [];

    let exit_code = commands[0].exec(&args, None, io_helper, dck_helper, dl_helper, log_helper);

    assert_eq!(exit_code, CommandExitCode::ConfigFileNotFound);
}

#[test]
fn check_if_need_config_file_and_found() {
    let io_helper = &TestInputOutputHelper::new();
    let dck_helper = &TestContainerHelper::new();
    let dl_helper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    let one_cmd = Command {
        name: "test",
        description: "It's a test",
        short_name: "tst",
        min_args: 0,
        max_args: 0,
        usage: "",
        need_config_file: true,
        exec_cmd: test_help,
    };

    let commands = &[one_cmd];

    let args = [];

    let cfg_file = String::from("config.yml");

    io_helper.files.borrow_mut().insert(cfg_file.clone(), String::from("download_dir: \"~/.d-sh/download\"\napplications_dir: \"~/.d-sh/applications\"\ndockerfile:\n  from: \"ubuntu:18.04\"\n  tag: \"d-base-image:v1.0.0\""));

    let exit_code = commands[0].exec(&args, Some(cfg_file), io_helper, dck_helper, dl_helper, log_helper);

    assert_eq!(exit_code, CommandExitCode::Ok);
}

#[test]
fn check_if_need_config_file_and_found_but_wrong_format() {
    let io_helper = &TestInputOutputHelper::new();
    let dck_helper = &TestContainerHelper::new();
    let dl_helper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    let one_cmd = Command {
        name: "test",
        description: "It's a test",
        short_name: "tst",
        min_args: 0,
        max_args: 0,
        usage: "",
        need_config_file: true,
        exec_cmd: test_help,
    };

    let commands = &[one_cmd];

    let args = [];

    let cfg_file = String::from("config.yml");

    // Create file
    io_helper
        .files
        .borrow_mut()
        .insert(cfg_file.clone(), String::from("tutu"));


    let exit_code = commands[0].exec(&args, Some(cfg_file), io_helper, dck_helper, dl_helper, log_helper);

    assert_eq!(exit_code, CommandExitCode::ConfigFileFormatWrong);
}
