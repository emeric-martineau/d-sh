//!
//! # D-SH a tool to create easy container's applications.
//!
//! Release under MIT License.
mod command;
mod help;
mod io;

use std::env;
use command::Command;
use command::check::CHECK;
use command::init::INIT;
use help::help;
use help::version;
use io::DefaultInputOutputHelper;

const ALL_COMMANDS: &'static [Command] = &[CHECK, INIT];

///
/// Main function of D-SH
///
fn main() {
    // Get command line options
    let args: Vec<String> = env::args().collect();
    // Default exit code
    let mut exit_code: i32 = 0;

    let println = &mut DefaultInputOutputHelper;

    if args.len() == 1 {
        help(ALL_COMMANDS, println);
        exit_code = 1
    } else {
        let command = &args[1];

        match command.as_str() {
            "-h" | "--help" => help(ALL_COMMANDS, println),
            "-v" | "--version" => version(&args, println),
            cmd => {
                let mut command_to_run = None;

                for c in ALL_COMMANDS {
                    if cmd == c.name {
                        command_to_run = Some(c);
                    }
                }

                exit_code = match command_to_run {
                    Some(c) => c.exec(&args, println),
                    None => {
                        eprintln!("D-SH: '{}' is not a d.sh command.", cmd);
                        eprintln!("See '{} --help'", args[0]);

                        2
                    }
                }
            }
        };
    }

    std::process::exit(exit_code)
}
