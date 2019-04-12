///
/// Module to display command.help.
///
/// Release under MIT License.
///
use command::{Command, CommandParameter, CommandError, ALL_COMMANDS};

#[cfg(test)]
mod tests;

///
/// Display command.help of D-SH.
///
/// `commands` parameter is list of all avaible command
///
fn help(cmd_param: CommandParameter) -> Result<(), CommandError> {
    cmd_param.io_helper.println(&format!(""));
    cmd_param.io_helper.println(&format!("Usage: d-sh COMMAND"));
    cmd_param.io_helper.println(&format!(""));
    cmd_param.io_helper.println(&format!("A tool to container all your life"));
    cmd_param.io_helper.println(&format!(""));
    cmd_param.io_helper.println(&format!("Options:"));
    cmd_param.io_helper.println(&format!(
        "  -d, --debug              Enabled debug mode"
    ));
    cmd_param.io_helper.println(&format!(
        "  -c, --config             Set config filename"
    ));
    cmd_param.io_helper.println(&format!(""));
    cmd_param.io_helper.println(&format!("Commands:"));

    for cmd in ALL_COMMANDS {
        let command = format!("{} ({})", cmd.name, cmd.short_name);
        cmd_param.io_helper.println(&format!(
            "  {:<width$}{}",
            command,
            cmd.description,
            width = 13
        ));
    }

    Ok(())
}

///
/// Display current version.
///
/// `args` parameter is command line arguments of D-SH.
///
fn version(cmd_param: CommandParameter) -> Result<(), CommandError> {
    let version = env!("CARGO_PKG_VERSION");

    let exe_path = match std::env::current_exe() {
        Ok(s) => match s.to_str() {
            Some(s) => s.to_string(),
            None => String::new()
        },
        Err(_) => String::new()
    };

    cmd_param.io_helper.println(&format!("{} version {}", exe_path, version));
    cmd_param.io_helper.println(&format!("Copyleft Emeric MARTINEAU (c) 2018-2019"));

    Ok(())
}

///
/// The `help` command.
///
pub const HELP: Command = Command {
    /// This command call by `check`.
    name: "help",
    /// description.
    description: "Print help",
    /// Short name.
    short_name: "h",
    /// `help` command have no parameter.
    min_args: 0,
    max_args: 0,
    /// `help` command have no command.help.
    usage: "",
    need_config_file: false,
    exec_cmd: help,
};

///
/// The `version` command.
///
pub const VERSION: Command = Command {
    /// This command call by `check`.
    name: "version",
    /// description.
    description: "Print version information and quit",
    /// Short name.
    short_name: "v",
    /// `help` command have no parameter.
    min_args: 0,
    max_args: 0,
    /// `help` command have no command.help.
    usage: "",
    need_config_file: false,
    exec_cmd: version,
};