///
/// Module to init config file.
///
/// Release under MIT License.
///
use command::Command;
use super::super::io::OutputWriter;
use std::env::home_dir;
use std::path::Path;

///
/// Function to implement check D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn init(command: &Command, args: &[String], writer: &mut OutputWriter) -> i32 {
    let mut exit_code = 0;

    match home_dir() {
        Some(path) => {
            let home_dir = match path.to_str() {
                None => String::from(""),
                Some(p) => {
                    let mut result = String::from(p);

                    if ! p.ends_with("/") {
                        result.push_str("/");
                    }

                    result
                }
            };

            let mut config_file = String::from(home_dir);
            config_file.push_str(".d-sh/config.yml");
println!("{}", &config_file);
            if Path::new(&config_file).exists() {
                println!("{}", "yes");
            } else {
                println!("{}", "no");
            }
        },
        None => {
            writer.eprintln("Impossible to get your home dir!");
            exit_code = 2;
        }
    }

    exit_code
}

///
/// The `check` command.
///
pub const INIT: Command = Command {
    /// This command call by `check`.
    name: "init",
    /// description.
    description: "Initialize config file if not exists",
    /// Short name.
    short_name: "it",
    /// `check` command have no parameter.
    min_args: 0,
    max_args: 0,
    /// `check` command have no help.
    usage: "",
    exec_cmd: init
};
