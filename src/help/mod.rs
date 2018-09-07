///
/// Module to display help.
///
/// Release under MIT License.
///
use super::command::Command;
use super::io::OutputWriter;

///
/// Display help of D-SH.
///
/// `commands` parameter is list of all avaible command
///
pub fn help(commands: &[Command], writer: &mut OutputWriter) {
    writer.println(&format!(""));
    writer.println(&format!("Usage: d-sh COMMAND"));
    writer.println(&format!(""));
    writer.println(&format!("A tool to container all your life"));
    writer.println(&format!(""));
    writer.println(&format!("Options:"));
    writer.println(&format!("  -h, --help               Print this current help"));
    writer.println(&format!("  -v, --version            Print version information and quit"));
    writer.println(&format!(""));
    writer.println(&format!("Commands:"));

    for cmd in commands {
        writer.println(&format!("  {:<width$}{}", cmd.name, cmd.description, width = 9));
    }
}

///
/// Display current version.
///
/// `args` parameter is command line arguments of D-SH.
///
pub fn version(args: &[String], writer: &mut OutputWriter) {
    let version = env!("CARGO_PKG_VERSION");

    writer.println(&format!("{} version {}", args[0], version));
    writer.println(&format!("Copyleft Emeric MARTINEAU (c) 2018"));
}

#[cfg(test)]
mod tests {
    use super::super::io::OutputWriter;
    use super::super::io::tests::TestOutputWriter;
    use super::version;
    use super::help;
    use super::super::command::Command;

    #[test]
    fn display_version() {
        let writer = &mut TestOutputWriter::new();

        let args = [String::from("ttt")];

        version(&args, writer);

        assert_eq!(writer.stdout.len(), 2);
    }

    fn test_help(command: &Command, args: &[String], writer: &mut OutputWriter) -> i32 {
        writer.println(&format!("Coucou !"));
        0
    }

    #[test]
    fn display_help() {
        let writer = &mut TestOutputWriter::new();

        let one_cmd = Command {
            name: "test",
            description: "It's a test",
            short_name: "tst",
            min_args: 0,
            max_args: 0,
            usage: "",
            exec_cmd: test_help
        };

        let commands = &[one_cmd];

        help(commands, writer);

        assert_eq!(writer.stdout.len(), 11);

        match writer.stdout.get(10) {
            Some(s) => assert_eq!(s, "  test     It's a test"),
            None => panic!("Help is not valid")
        }
    }
}
