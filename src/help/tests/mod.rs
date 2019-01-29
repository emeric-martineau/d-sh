///
/// Module to tests module help.
///
/// Release under MIT License.
///
use io::InputOutputHelper;
use io::tests::TestInputOutputHelper;
use super::{version, help};
use command::Command;
use docker::ContainerHelper;
use config::Config;
use command::CommandExitCode;
use download::DownloadHelper;

#[test]
fn display_version() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();

    let args = [String::from("ttt")];

    version(&args, io_helper);

    assert_eq!(io_helper.stdout.borrow().len(), 2);
}

fn test_help(_command: &Command, _args: &[String], io_helper: &InputOutputHelper,
    _dck_helper: &ContainerHelper, _dl_helper: &DownloadHelper,
    _config: Option<&Config>) -> CommandExitCode {
    io_helper.println(&format!("Coucou !"));
    CommandExitCode::Ok
}

#[test]
fn display_help() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();

    let one_cmd = Command {
        name: "test",
        description: "It's a test",
        short_name: "tst",
        min_args: 0,
        max_args: 0,
        usage: "",
        need_config_file: false,
        exec_cmd: test_help
    };

    let commands = &[one_cmd];

    help(commands, io_helper);

    let stdout = io_helper.stdout.borrow();

    assert_eq!(stdout.len(), 11);

    match stdout.get(10) {
        Some(s) => assert_eq!(s, "  test     It's a test"),
        None => panic!("Help is not valid")
    }
}
