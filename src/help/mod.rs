///
/// Module to display help.
///
/// Release under MIT License.
///
use command::Command;
use io::InputOutputHelper;

#[cfg(test)]
mod tests;

///
/// Display help of D-SH.
///
/// `commands` parameter is list of all avaible command
///
pub fn help(commands: &[Command], io_helper: &InputOutputHelper) {
    io_helper.println(&format!(""));
    io_helper.println(&format!("Usage: d-sh COMMAND"));
    io_helper.println(&format!(""));
    io_helper.println(&format!("A tool to container all your life"));
    io_helper.println(&format!(""));
    io_helper.println(&format!("Options:"));
    io_helper.println(&format!(
        "  -h, --help               Print this current help"
    ));
    io_helper.println(&format!(
        "  -v, --version            Print version information and quit"
    ));
    io_helper.println(&format!(""));
    io_helper.println(&format!("Commands:"));

    for cmd in commands {
        let command = format!("{} ({})", cmd.name, cmd.short_name);
        io_helper.println(&format!(
            "  {:<width$}{}",
            command,
            cmd.description,
            width = 13
        ));
    }
}

///
/// Display current version.
///
/// `args` parameter is command line arguments of D-SH.
///
pub fn version(args: &[String], io_helper: &InputOutputHelper) {
    let version = env!("CARGO_PKG_VERSION");

    io_helper.println(&format!("{} version {}", args[0], version));
    io_helper.println(&format!("Copyleft Emeric MARTINEAU (c) 2018"));
}
