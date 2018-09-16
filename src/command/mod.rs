///
/// Module with command module.
///
/// Release under MIT License.
///
pub mod check;
pub mod init;

use super::io::InputOutputHelper;


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
    /// Execute Command.
    pub exec_cmd: fn(command: &Command, args: &[String], io_helper: &mut InputOutputHelper) -> i32
}

impl Command {
    ///
    /// Execute code of command
    ///
    /// `args` parameter is command line arguments of D-SH.
    ///
    /// returning exit code of D-SH
    ///
    pub fn exec(&self, args: &[String], io_helper: &mut InputOutputHelper) -> i32 {
        // Check parameter
        if args.len() >= self.min_args && args.len() <= self.max_args {
            (self.exec_cmd)(self, &args, io_helper)
        } else {
            io_helper.eprintln(&format!("\"d-sh {}\" bad arguments number.", self.name));
            io_helper.eprintln("See 'd-sh $1 --help'.");

            4
        }
    }
}
