///
/// Module to check build container.
///
/// Release under MIT License.
///
use command::Command;
use super::super::io::InputOutputHelper;

///
/// Function to implement check D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn check(command: &Command, args: &[String], io_helper: &mut InputOutputHelper) -> i32 {


    io_helper.println(&format!("Coucou !"));
    0
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
