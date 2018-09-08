///
/// Module with command module.
///
/// Release under MIT License.
///
pub mod check;
pub mod init;

use super::io::OutputWriter;


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
    pub min_args: u8,
    /// Maximum arguments of command.
    pub max_args: u8,
    /// Usage for help command.
    pub usage: &'static str,
    /// Execute Command.
    pub exec_cmd: fn(command: &Command, args: &[String], println: &mut OutputWriter) -> i32
}

impl Command {
    ///
    /// Execute code of command
    ///
    /// `args` parameter is command line arguments of D-SH.
    ///
    /// returning exit code of D-SH
    ///
    pub fn exec(&self, args: &[String], println: &mut OutputWriter) -> i32 {
        (self.exec_cmd)(self, &args, println)
    }
}
