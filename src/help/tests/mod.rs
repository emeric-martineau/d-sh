///
/// Module to tests module help.
///
/// Release under MIT License.
///
use io::tests::TestInputOutputHelper;
use super::{version, help};
use command::{Command, CommandError, CommandParameter};

#[test]
fn display_version() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();

    let args = [String::from("ttt")];

    version(&args, io_helper);

    assert_eq!(io_helper.stdout.borrow().len(), 2);
}

fn test_help(cmd_param: CommandParameter) -> Result<(), CommandError> {
    cmd_param.io_helper.println(&format!("Coucou !"));
    Ok(())
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
        Some(s) => assert_eq!(s, "  test (tst)   It's a test"),
        None => panic!("Help is not valid")
    }
}
