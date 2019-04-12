use super::{help, version, VERSION, HELP};
use command::CommandParameter;
use command::tests::{test_result_ok};
///
/// Module to tests module command.help.
///
/// Release under MIT License.
///
use io::tests::TestInputOutputHelper;
use docker::tests::TestContainerHelper;
use download::tests::TestDownloadHelper;
use log::{EmptyLoggerHelper};

#[test]
fn display_version() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    let args = [String::from("ttt")];

    let cmd_param = CommandParameter {
        command: &VERSION,
        args: &args,
        io_helper,
        dck_helper,
        dl_helper,
        log_helper,
        config: None,
        config_filename: String::new()
    };

    test_result_ok(version(cmd_param));

    assert_eq!(io_helper.stdout.borrow().len(), 2);
}

#[test]
fn display_help() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    let args = [String::from("ttt")];

    let cmd_param = CommandParameter {
        command: &HELP,
        args: &args,
        io_helper,
        dck_helper,
        dl_helper,
        log_helper,
        config: None,
        config_filename: String::new()
    };

    test_result_ok(help(cmd_param));

    let stdout = io_helper.stdout.borrow();

    assert_eq!(stdout.len(), 18);
}
